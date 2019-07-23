use serde::Serialize;

use super::http_client::hyper::HyperClient;
use super::http_client::{Body, HttpClient};
use super::middleware::{Middleware, Next};
use super::Exception;
use super::Response;

use std::convert::TryInto;
use std::fmt;
use std::sync::Arc;

/// Create an HTTP request.
pub struct Request {
    client: hyper::client::Builder,
    method: http::Method,
    headers: http::HeaderMap,
    middleware: Option<Vec<Arc<dyn Middleware>>>,
    uri: http::Uri,
    body: Body,
}

impl fmt::Debug for Request {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Request")
            .field("client", &self.client)
            .finish()
    }
}

impl Request {
    /// Create a new instance.
    pub fn new(method: http::Method, uri: http::Uri) -> Self {
        Self {
            client: hyper::client::Client::builder(),
            body: Body::empty(),
            headers: http::HeaderMap::new(),
            middleware: Some(vec![]),
            method,
            uri,
        }
    }

    /// Push middleware onto the middleware stack.
    pub fn middleware(mut self, mw: impl Middleware) -> Self {
        // We can safely unwrap here because middleware is only ever set to None
        // in the finalizing `send` method.
        self.middleware.as_mut().unwrap().push(Arc::new(mw));
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
    pub async fn form(self) -> Result<(), Exception> {
        // let mut _res = self.send().await?;
        unimplemented!();
    }

    /// Send the request and get back a response.
    pub async fn send(mut self) -> Result<Response, Exception> {
        // We can safely unwrap here because this is the only time we take ownership of the
        // middleware stack.
        let middleware = self.middleware.take().unwrap();

        let next = Next::new(&middleware, &|req| {
            Box::pin(async move { HyperClient::new().send(req).await.map_err(|e| e.into()) })
        });

        let req = self.try_into()?;
        let res = next.run(req).await?;
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
