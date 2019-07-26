use futures::future::BoxFuture;
use futures::prelude::*;
use serde::Serialize;

use super::http_client::hyper::HyperClient;
use super::http_client::{Body, HttpClient};
use super::middleware::{Middleware, Next};
use super::Exception;
use super::Response;

use std::convert::{TryInto, TryFrom};
use std::fmt;
use std::fmt::Debug;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use std::io;

struct RequestState<C: HttpClient> {
    method: http::Method,
    headers: http::HeaderMap,
    middleware: Option<Vec<Arc<dyn Middleware<C>>>>,
    uri: http::Uri,
    body: Body,
}

impl<C: HttpClient> fmt::Debug for RequestState<C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RequestState")
            .field("method", &self.method)
            .field("uri", &self.uri)
            .field("middleware", &"[<middleware>]")
            .field("body", &"<body>")
            .finish()
    }
}

/// Create an HTTP request.
pub struct Request<C: HttpClient + Debug + Unpin + Send + Sync> {
    /// Holds a `http_client::HttpClient` implementation
    client: Option<C>,
    /// Holds the state of the request
    req: Option<RequestState<C>>,
    /// Holds the state of the `impl Future`
    fut: Option<BoxFuture<'static, Result<Response, Exception>>>,
}

impl Request<HyperClient> {
    /// Create a new instance.
    pub fn new(method: http::Method, uri: http::Uri) -> Self {
        Self::with_client(method, uri, HyperClient::new())
    }
}

impl<C: HttpClient> Request<C> {
    /// Create a new instance with an `HttpClient` instance.
    pub fn with_client(method: http::Method, uri: http::Uri, client: C) -> Self {
        let client = Self {
            fut: None,
            client: Some(client),
            req: Some(RequestState {
                body: Body::empty(),
                headers: http::HeaderMap::new(),
                middleware: Some(vec![]),
                method,
                uri,
            }),
        };

        #[cfg(feature="middleware-logger")]
        let client = client.middleware(crate::middleware::logger::new());

        client
    }

    /// Push middleware onto the middleware stack.
    pub fn middleware(mut self, mw: impl Middleware<C>) -> Self {
        self.req
            .as_mut()
            .unwrap()
            .middleware
            .as_mut()
            .unwrap()
            .push(Arc::new(mw));
        self
    }

    /// Insert a header.
    pub fn header(
        mut self,
        key: impl http::header::IntoHeaderName,
        value: impl AsRef<str>,
    ) -> Self {
        let value = value.as_ref().to_owned();
        self.req
            .as_mut()
            .unwrap()
            .headers
            .insert(key, value.parse().unwrap());
        self
    }

    /// Pass an `AsyncRead` stream as the request body.
    pub fn body<R: AsyncRead + Unpin + Send + 'static>(mut self, reader: Box<R>) -> Self{
        self.req.as_mut().unwrap().body = reader.into();
        self
    }

    /// Set JSON as the body.
    pub fn json<T: Serialize>(mut self, json: &T) -> serde_json::Result<Self> {
        self.req.as_mut().unwrap().body = serde_json::to_vec(json)?.into();
        let content_type = "application/json".parse().unwrap();
        self.req
            .as_mut()
            .unwrap()
            .headers
            .insert("content-type", content_type);
        Ok(self)
    }

    /// Submit the request and get the response body as bytes.
    pub async fn recv_bytes(self) -> Result<Vec<u8>, Exception> {
        let mut req = self.await?;
        Ok(req.body_bytes().await?)
    }

    /// Submit the request and get the response body as a string.
    pub async fn recv_string(self) -> Result<String, Exception> {
        let mut req = self.await?;
        Ok(req.body_string().await?)
    }

    /// Submit the request and get the response body as a string.
    pub async fn recv_json<T: serde::de::DeserializeOwned>(self) -> Result<T, Exception> {
        let mut req = self.await?;
        Ok(req.body_json::<T>().await?)
    }
}

impl<C: HttpClient> Future for Request<C> {
    type Output = Result<Response, Exception>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let None = self.fut {
            // We can safely unwrap here because this is the only time we take ownership of the
            // request and middleware stack.
            let mut req = self.req.take().unwrap();
            let client = self.client.take().unwrap();
            let middleware = req.middleware.take().unwrap();
            let req = req.try_into()?;

            self.fut = Some(Box::pin(async move {
                let next = Next::new(&middleware, &|req, client| {
                    Box::pin(async move { client.send(req).await.map_err(|e| e.into()) })
                });

                let res = next.run(req, client).await?;
                Ok(Response::new(res))
            }));
        }

        self.fut.as_mut().unwrap().as_mut().poll(cx)
    }
}

impl<C: HttpClient> TryInto<http::Request<Body>> for RequestState<C> {
    type Error = http::Error;

    fn try_into(self) -> Result<http::Request<Body>, Self::Error> {
        let res = http::Request::builder()
            .method(self.method)
            .uri(self.uri)
            .body(self.body)?;
        Ok(res)
    }
}

impl<R: AsyncRead + Unpin + Send + 'static> TryFrom<http::Request<Box<R>>> for Request<HyperClient> {
    type Error = io::Error;

    /// Converts an `http::Request` to a `surf::Request`.
    fn try_from(http_request: http::Request<Box<R>>) -> io::Result<Self> {
        let (parts, body) = http_request.into_parts();
        let req = Self::new(parts.method, parts.uri);
        let req = req.body(Box::new(Body::from(body)));
        Ok(req)
    }
}

impl<C: HttpClient> TryInto<http::Request<Body>> for Request<C> {
    type Error = http::Error;

    /// Converts a `surf::Request` to an `http::Request`.
    fn try_into(self) -> Result<http::Request<Body>, Self::Error> {
        self.req.unwrap().try_into()
    }
}

impl<C: HttpClient> fmt::Debug for Request<C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.req, f)
    }
}
