use cacache;
use futures::prelude::*;
use futures::future::BoxFuture;
use http::{HeaderMap, Request};
use hyper;
use serde_json::{Map, Number, Value};

use crate::middleware::{Next, Middleware, Request, Response, HttpClient};
use crate::Exception;

pub fn has_cond_header(headers: &HeaderMap) -> bool {
    for key in headers.keys() {
        let key = key.as_str().to_lowercase();
        if [
            "if-modified-since",
            "if-none-match",
            "if-unmodified-since",
            "if-match",
            "if-range",
        ]
        .iter()
        .any(|x| String::from(*x) == key)
        {
            return true;
        }
    }
    false
}

fn cache_key<T>(req: &Request<T>) -> String {
    format!("surf:req:v1:{}", req.uri())
}

#[derive(Debug, PartialEq)]
/// Cache modes, based on JavaScript's `fetch()` cache modes.
pub enum CacheMode {
    Default,
    NoStore,
    NoCache,
    Reload,
    ForceCache,
    OnlyIfCached,
}

#[derive(Debug)]
pub struct Cacache {
    mode: CacheMode,
    path: String,
}

impl Cacache {
    pub async fn matched<T, U>(&self, req: &Request<T>) -> Result<Option<Response<U>>, Fail>
    where
        U: AsyncRead,
    {
        if let Some(entry) = cacache::get::info(&self.path, cache_key(&req))? {
            let mut res = hyper::Response::builder();
            // TODO - convert the unwrap()s to .ok_or()
            res.status(entry.metadata["status"].as_u64().unwrap() as u16);
            let headers = entry.metadata["headers"].as_object().unwrap();
            for (header, value) in headers.into_iter() {
                res.header(header.as_str(), value.as_str().unwrap());
            }
            Ok(Some(Response::new(
                res.body(hyper::Body::empty())?,
                cacache::async_get::open_hash(&self.path, entry.integrity).await?,
            )))
        } else {
            Ok(None)
        }
    }

    pub async fn put<T, U>(&self, req: Request<T>, res: Response<U>) -> Result<Response<U>, Fail>
    where
        U: AsyncRead + Unpin,
    {
        let mut metadata = Map::new();
        metadata.insert(
            "status".into(),
            Value::Number(Number::from(res.response.status().as_u16())),
        );
        let mut headers = Map::new();
        for (key, value) in res.response.headers().iter() {
            headers.insert(key.as_str().into(), Value::String(value.to_str()?.into()));
        }
        metadata.insert("headers".into(), Value::Object(headers));
        let metadata = Value::Object(metadata);
        let put = cacache::put::PutOpts::new()
            .metadata(metadata)
            .open_async(&self.path, cache_key(&req))
            .await?;
        let mut buf = [0; 1024 * 256];
        loop {
            let amt = res.read(&mut buf).await?;
            if amt == 0 {
                break;
            } else {
                put.write_all(&buf[0..amt]).await?;
            }
        }
        put.commit().await?;
        let res = self.matched(&req).await?.unwrap();
        Ok(res)
    }

    pub async fn delete<T>(&self, req: Request<T>) -> Result<bool, Fail> {
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
    fn send_remote(
        self,
        req: hyper::Request<hyper::Body>,
    ) -> BoxFuture<Result<Response<Box<impl futures::io::AsyncRead>>, Fail>> {
        Box::pin(async move {
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
        })
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
    pub async fn send<'a, C: HttpClient>(
        mut self,
        req: Request,
        client: C,
        next: Next<'a, C>,
     ) -> Result<Response<Box<impl futures::io::AsyncRead>>, Exception> {
        let mut headers = req.headers_mut();
        for (key, value) in self.headers.iter() {
            headers.insert(key, *value);
        }
        if self.cache_mode == CacheMode::Default && has_cond_header(&headers) {
            self.cache_mode = CacheMode::NoStore;
        }
        if self.request_cacheable(&req) {
            if let Some(c) = self.cache {
                if let Some(mut res) = c.matched(&req).await? {
                    let mut res_headers = res.response.headers_mut();
                    // TODO - this needs to handle multiple potential Warning
                    // headers
                    if let Some(warning) = res_headers.get("warning".into()) {
                        let code: String = warning
                            .to_str()
                            .unwrap_or("000")
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

impl<C: HttpClient> Middleware<C> for Cacache {
    fn handle<'a>(
        &'a self,
        req: Request,
        client: C,
        next: Next<'a, C>,
    ) -> BoxFuture<'a, Result<Response, Exception>> {
        Box::pin(async move {
            self.send(req, client, next).await
        })
    }
}
