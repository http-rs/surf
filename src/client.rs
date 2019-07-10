use futures::compat::Compat01As03;
use futures::prelude::*;
use serde::Serialize;
use std::io;

use super::Fail;

/// Create an HTTP request.
#[derive(Debug)]
pub struct Client {
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
            client: hyper::client::Client::builder(),
            body: hyper::Body::empty(),
            headers: http::HeaderMap::new(),
            method,
            uri,
        }
    }

    /// Set JSON as the body.
    pub fn json<T: Serialize>(mut self, json: &T) -> Result<Self, Fail> {
        self.body = serde_json::to_vec(json)?.into();
        let content_type = "application/json".parse().unwrap();
        self.headers.insert("content-type", content_type);
        Ok(self)
    }

    /// Send a request and format the response as a `FormData`.
    pub async fn form(self) -> Result<(), Fail> {
        let mut _res = self.send().await?;
        unimplemented!();
    }

    /// Send th request and get back a response.
    pub async fn send(self) -> Result<Response, Fail> {
        let req = hyper::Request::builder()
            .method(self.method)
            .uri(self.uri)
            .body(self.body)?;
        let client = hyper::Client::new();
        let res = Compat01As03::new(client.request(req)).await?;
        Ok(Response::new(res))
    }
}
