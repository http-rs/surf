use futures::future::BoxFuture;
use serde::Serialize;
use futures::future::BoxFuture;

use super::http_client::hyper::HyperClient;
use super::http_client::{Body, HttpClient};
use super::middleware::{Middleware, Next, self};
use super::Exception;
use super::Response;

use std::convert::TryInto;
use std::fmt;
use std::sync::Arc;
use std::task::{Context, Poll};
use std::pin::Pin;
use std::future::Future;

struct RequestState {
    client: hyper::client::Builder,
    method: http::Method,
    headers: http::HeaderMap,
    middleware: Vec<Arc<dyn Middleware>>,
    uri: http::Uri,
    body: Body,
}

impl fmt::Debug for RequestState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Request")
            .field("client", &self.client)
            .field("method", &self.method)
            .field("uri", &self.uri)
            .field("body", &"<body>")
            .finish()
    }
}

/// Create an HTTP request.
pub struct Request {
    /// Holds the state of the request
    req: RequestState,
    /// Holds the state of the `impl Future`
    fut: Option<BoxFuture<'static, Result<middleware::Response, Exception>>>,
}

impl Request {
    /// Create a new instance.
    pub fn new(method: http::Method, uri: http::Uri) -> Self {
        Self {
            fut: None,
            req: RequestState {
                client: hyper::client::Client::builder(),
                body: Body::empty(),
                headers: http::HeaderMap::new(),
                middleware: vec![],
                method,
                uri,
            }
        }
    }

    /// Push middleware onto the middleware stack.
    pub fn middleware(mut self, mw: impl Middleware) -> Self {
        self.req.middleware.push(Arc::new(mw));
        self
    }

    /// Insert a header.
    pub fn header(
        mut self,
        key: impl http::header::IntoHeaderName,
        value: impl AsRef<str>,
    ) -> Self {
        let value = value.as_ref().to_owned();
        self.req.headers.insert(key, value.parse().unwrap());
        self
    }

    /// Set JSON as the body.
    pub fn json<T: Serialize>(mut self, json: &T) -> serde_json::Result<Self> {
        self.req.body = serde_json::to_vec(json)?.into();
        let content_type = "application/json".parse().unwrap();
        self.req.headers.insert("content-type", content_type);
        Ok(self)
    }

    /// Send a request and format the response as a `FormData`.
    pub async fn form(self) -> Result<(), Exception> {
        // let mut _res = self.send().await?;
        unimplemented!();
    }

    // /// Send the request and get back a response.
    // pub async fn send(mut self) -> Result<Response, Exception> {
    //     // We can safely unwrap here because this is the only time we take ownership of the
    //     // middleware stack.
    //     let middleware = self.req.middleware.take().unwrap();

    //     let next = Next::new(&middleware, &|req| {
    //         Box::pin(async move { HyperClient::new().send(req).await.map_err(|e| e.into()) })
    //     });

    //     let req = self.try_into()?;
    //     let res = next.run(req).await?;
    //     Ok(Response::new(res))
    // }
}

impl Future for Request {
    type Output = Result<Response, Exception>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let None = self.fut {
            let next = Next::new(&self.req.middleware, &|req| {
                Box::pin(async move { HyperClient::new().send(req).await.map_err(|e| e.into()) })
            });

            let req = self.req.try_into()?;
            self.fut = Some(next.run(req));
        }

        // let res = futures::ready!(self.fut.unwrap().run(req))?;
        // Poll::Ready(Ok(Response::new(res)))
        unimplemented!();
    }
}

impl TryInto<http::Request<Body>> for RequestState {
    type Error = http::Error;

    fn try_into(self) -> Result<http::Request<Body>, Self::Error> {
        let res = http::Request::builder()
            .method(self.method)
            .uri(self.uri)
            .body(self.body)?;
        Ok(res)
    }
}

impl fmt::Debug for Request {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.req, f)
    }
}
