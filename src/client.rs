use futures::compat::Compat01As03;
use serde::Serialize;

use super::cache::{self, Cacache, CacheMode};
use super::Fail;
use super::Response;

/// Create an HTTP request.
#[derive(Debug)]
pub struct Client {
    cache: Option<Cacache>,
    cache_mode: CacheMode,
    client: hyper::client::Builder,
    method: http::Method,
    headers: http::HeaderMap,
    uri: http::Uri,
    body: hyper::Body,
}

impl Client {
    /// Create a new instance.
    pub fn new(method: http::Method, uri: http::Uri) -> Self {
        Self {
            cache: None,
            cache_mode: CacheMode::Default,
            client: hyper::client::Client::builder(),
            body: hyper::Body::empty(),
            headers: http::HeaderMap::new(),
            method,
            uri,
        }
    }

    /// Configure a cache.
    pub fn cache(mut self, mode: CacheMode, cache: Cacache) -> Self {
        self.cache = Some(cache);
        self.cache_mode = mode;
        self
    }

    /// Insert a header.
    pub fn header(
        mut self,
        key: impl http::header::IntoHeaderName,
        value: impl AsRef<str>,
    ) -> Self {
        let value = value.as_ref().to_owned();
        self.headers.insert(key, value.parse().unwrap());
        self
    }

    /// Set JSON as the body.
    pub fn json<T: Serialize>(mut self, json: &T) -> serde_json::Result<Self> {
        self.body = serde_json::to_vec(json)?.into();
        let content_type = "application/json".parse().unwrap();
        self.headers.insert("content-type", content_type);
        Ok(self)
    }

    /// Send a request and format the response as a `FormData`.
    pub async fn form(self) -> Result<(), Fail> {
        // let mut _res = self.send().await?;
        unimplemented!();
    }

    fn request_cacheable<T>(&self, req: &hyper::Request<T>) -> bool {
        let method = *req.method();
        (method == http::Method::GET || method == http::Method::HEAD)
            && self.cache_mode != CacheMode::NoStore
            && self.cache_mode != CacheMode::Reload
    }

    fn response_cacheable<T>(&self, res: &Response<T>) -> bool {
        unimplemented!();
    }

    fn is_stale<T>(&self, req: &Response<T>) -> bool {
        unimplemented!();
    }

    fn set_warning<T>(&self, res: &Response<T>, code: u32, msg: &str) {
        unimplemented!();
        //   Warning    = "Warning" ":" 1#warning-value
        // warning-value = warn-code SP warn-agent SP warn-text [SP warn-date]
        // warn-code  = 3DIGIT
        // warn-agent = ( host [ ":" port ] ) | pseudonym
        //                 ; the name or pseudonym of the server adding
        //                 ; the Warning header, for use in debugging
        // warn-text  = quoted-string
        // warn-date  = <"> HTTP-date <">
        // (https://tools.ietf.org/html/rfc2616#section-14.46)
        //   const host = url.parse(reqOrRes.url).host
        //   const jsonMessage = JSON.stringify(message)
        //   const jsonDate = JSON.stringify(new Date().toUTCString())
        //   const header = replace ? 'set' : 'append'

        //   reqOrRes.headers[header](
        //     'Warning',
        //     `${code} ${host} ${jsonMessage} ${jsonDate}`
        //   )
        // );
    }

    /// Send a remote request without consulting the cache
    async fn send_remote(
        self,
        req: hyper::Request<hyper::Body>,
    ) -> Result<Response<Box<impl futures::io::AsyncRead>>, Fail> {
        use futures::prelude::*;
        use std::io;
        let client = hyper::Client::new();
        let mut res = Compat01As03::new(client.request(req)).await?;
        let body = std::mem::replace(res.body_mut(), hyper::Body::empty());
        let body = Box::new(
            Compat01As03::new(body)
                .map(|chunk| chunk.map(|chunk| chunk.to_vec()))
                .map_err(|_| io::ErrorKind::InvalidData.into())
                .into_async_read(),
        );
        Ok(Response::new(res, body))
    }

    /// Send a conditional request or return the original cached response
    async fn send_conditional<T>(
        self,
        req: hyper::Request<hyper::Body>,
        cachedRes: Response<T>,
    ) -> Result<Response<Box<impl futures::io::AsyncRead>>, Fail>
    where
        T: futures::io::AsyncRead,
    {
        // TODO -
        unimplemented!();
    }

    /// Send the request and get back a response.
    pub async fn send(mut self) -> Result<Response<Box<impl futures::io::AsyncRead>>, Fail> {
        let mut req = hyper::Request::builder()
            .method(self.method)
            .uri(self.uri)
            .body(self.body)?;
        let mut headers = req.headers_mut();
        for (key, value) in self.headers.iter() {
            headers.insert(key, *value);
        }
        if self.cache_mode == CacheMode::Default && cache::has_cond_header(&headers) {
            self.cache_mode = CacheMode::NoStore;
        }
        if self.request_cacheable(&req) {
            if let Some(c) = self.cache {
                if let Some(mut res) = c.matched(&req).await? {
                    let mut res_headers = res.response.headers_mut();
                    if let Some(warning) = res_headers.get("warning".into()) {
                        let code: String = warning
                            .to_str()?
                            .chars()
                            .take_while(|x| (*x).is_digit(10))
                            .collect();
                        let code: u32 = code.parse()?;
                        // https://tools.ietf.org/html/rfc7234#section-4.3.4
                        //
                        // If a stored response is selected for update, the cache
                        // MUST:
                        //
                        // * delete any Warning header fields in the stored
                        //   response with warn-code 1xx (see Section 5.5);
                        //
                        // * retain any Warning header fields in the stored
                        //   response with warn-code 2xx;
                        //
                        if code >= 100 && code < 200 {
                            res_headers.remove("warning".into());
                        }
                    }
                    if self.cache_mode == CacheMode::Default && !self.is_stale(&res) {
                        return Ok(res);
                    }

                    if self.cache_mode == CacheMode::Default
                        || self.cache_mode == CacheMode::NoCache
                    {
                        return self.send_conditional(req, res).await;
                    }

                    if self.cache_mode == CacheMode::ForceCache
                        || self.cache_mode == CacheMode::OnlyIfCached
                    {
                        //   112 Disconnected operation
                        //
                        //   SHOULD be included if the cache is intentionally
                        //   disconnected from the rest of the network for a
                        //   period of time.
                        //   (https://tools.ietf.org/html/rfc2616#section-14.46)
                        self.set_warning(&res, 112, "Disconnected operation");
                        return Ok(res);
                    }
                } else if self.cache_mode == CacheMode::OnlyIfCached {
                    // TODO - do an error instead
                    panic!("cache mode is OnlyIfCached, but no cached response found");
                }
            }
        }
        let maybe_cache = self.cache.take();
        let res = self.send_remote(req).await?;
        if self.response_cacheable(&res) {
            if let Some(c) = maybe_cache {
                return c.put(req, res).await;
            }
        }
        Ok(res)
    }
}
