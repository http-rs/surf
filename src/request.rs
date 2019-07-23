use serde::Serialize;
use std::convert::TryInto;

use super::http_client::{Body, HttpClient};
use super::http_client_hyper::HyperClient;
use super::Fail;
use super::Response;

/// Create an HTTP request.
#[derive(Debug)]
pub struct Request {
    client: hyper::client::Builder,
    method: http::Method,
    headers: http::HeaderMap,
    uri: http::Uri,
    body: Body,
}

impl Request {
    /// Create a new instance.
    pub fn new(method: http::Method, uri: http::Uri) -> Self {
        Self {
            client: hyper::client::Client::builder(),
            body: Body::empty(),
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

    /// Send the request and get back a response.
    pub async fn send(self) -> Result<Response, Fail> {
        let req = self.try_into()?;
        let res = HyperClient::new().send(req).await?;
        Ok(Response::new(res))
    }
}

impl TryInto<http::Request<Body>> for Request {
    type Error = http::Error;

    fn try_into(self) -> Result<http::Request<Body>, Self::Error> {
        let res = http::Request::builder()
            .method(self.method)
            .uri(self.uri)
            .body(self.body)?;
        Ok(res)
    }
}
