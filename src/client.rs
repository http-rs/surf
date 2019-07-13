use futures::compat::Compat01As03;
use serde::Serialize;

use super::cache::{self, Cacache};
use super::Fail;
use super::Response;

/// Create an HTTP request.
#[derive(Debug)]
pub struct Client {
    cache: Option<Cacache>,
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
            client: hyper::client::Client::builder(),
            body: hyper::Body::empty(),
            headers: http::HeaderMap::new(),
            method,
            uri,
        }
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

    /// Send th request and get back a response.
    pub async fn send(self) -> Result<Response<Box<impl futures::io::AsyncRead>>, Fail> {
        use futures::prelude::*;
        use std::io;
        let req = hyper::Request::builder()
            .method(self.method)
            .uri(self.uri)
            .body(self.body)?;

        if cache::is_cacheable(&req) {
            if let Some(c) = self.cache {
                if let Some(res) = c.matched(&req).await? {
                    return Ok(res);
                }
            }
        }
        let client = hyper::Client::new();
        let mut res = Compat01As03::new(client.request(req)).await?;
        let body = std::mem::replace(res.body_mut(), hyper::Body::empty());
        let body = Box::new(
            Compat01As03::new(body)
                .map(|chunk| chunk.map(|chunk| chunk.to_vec()))
                .map_err(|_| io::ErrorKind::InvalidData.into())
                .into_async_read(),
        );
        let res = Response::new(res, body);
        if cache::is_cacheable(&req) {
            if let Some(c) = self.cache {
                return c.put(req, res).await;
            }
        }
        Ok(res)
    }
}
