use futures::future::BoxFuture;
use futures::prelude::*;
use serde::Serialize;
use http::Method;
use url::Url;
use mime::Mime;

use super::http_client::hyper::HyperClient;
use super::http_client::{self, Body, HttpClient};
use super::middleware::{Middleware, Next};
use super::Exception;
use super::Response;

use std::convert::TryFrom;
use std::fmt;
use std::fmt::Debug;
use std::future::Future;
use std::io;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

/// Create an HTTP request.
pub struct Request<C: HttpClient + Debug + Unpin + Send + Sync> {
    /// Holds a `http_client::HttpClient` implementation.
    client: Option<C>,
    /// Holds the state of the request.
    req: Option<http_client::Request>,
    /// Holds the inner middleware.
    middleware: Option<Vec<Arc<dyn Middleware<C>>>>,
    /// Holds the state of the `impl Future`.
    fut: Option<BoxFuture<'static, Result<Response, Exception>>>,
    /// Holds a reference to the Url
    url: Url,
}

impl Request<HyperClient> {
    /// Create a new instance.
    pub fn new(method: http::Method, url: Url) -> Self {
        Self::with_client(method, url, HyperClient::new())
    }
}

impl<C: HttpClient> Request<C> {
    /// Create a new instance with an `HttpClient` instance.
    pub fn with_client(method: http::Method, url: Url, client: C) -> Self {
        let mut req = http_client::Request::new(Body::empty());
        *req.method_mut() = method;
        *req.uri_mut() = url.as_str().parse().unwrap();
        let client = Self {
            fut: None,
            client: Some(client),
            req: Some(req),
            url: url,
            middleware: Some(vec![]),
        };

        #[cfg(feature="middleware-logger")]
        let client = client.middleware(crate::middleware::logger::new());

        client
    }

    /// Push middleware onto the middleware stack.
    pub fn middleware(mut self, mw: impl Middleware<C>) -> Self {
        self.middleware.as_mut().unwrap().push(Arc::new(mw));
        self
    }

    /// Get an HTTP header.
    pub fn header(&self, key: &'static str) -> Option<&'_ str> {
        let req = self.req.as_ref().unwrap();
        req.headers().get(key).map(|h| h.to_str().unwrap())
    }

    /// Insert an HTTP header.
    pub fn set_header(mut self, key: &'static str, value: impl AsRef<str>) -> Self {
        let value = value.as_ref().to_owned();
        let req = self.req.as_mut().unwrap();
        req.headers_mut().insert(key, value.parse().unwrap());
        self
    }

    /// Get the request HTTP method.
    pub fn method(&self) -> &Method {
        let req = self.req.as_ref().unwrap();
        req.method()
    }

    /// Get the request url.
    pub fn url(&self) -> &Url {
        &self.url
    }

    /// Get the request MIME.
    ///
    /// Gets the `Content-Type` header and parses it to a `Mime` type.
    ///
    /// # Panics
    /// This method will panic if an invalid MIME type was set as a header.
    ///
    /// [Read more on MDN](https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types)
    pub fn mime(&self) -> Option<Mime> {
        let header = self.header("Content-Type")?;
        Some(header.parse().unwrap())
    }

    /// Set the request MIME.
    ///
    /// [Read more on MDN](https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types)
    pub fn set_mime(self, mime: Mime) -> Self {
        self.set_header("Content-Type", format!("{}", mime))
    }

    /// Pass an `AsyncRead` stream as the request body.
    ///
    /// # Mime
    /// The encoding is set to `application/octet-stream`.
    pub fn set_body<R>(mut self, reader: Box<R>) -> Self where R: AsyncRead + Unpin + Send + 'static{
        *self.req.as_mut().unwrap().body_mut() = reader.into();
        self.set_mime(mime::APPLICATION_OCTET_STREAM)
    }

    /// Set JSON as the request body.
    ///
    /// # Mime
    /// The encoding is set to `application/json`.
    ///
    /// # Errors
    /// This method will return an error if the provided data could not be serialized to JSON.
    pub fn set_json(mut self, json: &impl Serialize) -> serde_json::Result<Self> {
        *self.req.as_mut().unwrap().body_mut() = serde_json::to_vec(json)?.into();
        Ok(self.set_mime(mime::APPLICATION_JSON))
    }

    /// Set a string as the request body.
    ///
    /// # Mime
    /// The encoding is set to `text/plain; charset=utf-8`.
    ///
    /// # Errors
    /// This method will return an error if the provided data could not be serialized to JSON.
    pub fn set_string(mut self, string: String) -> serde_json::Result<Self> {
        *self.req.as_mut().unwrap().body_mut() = string.into_bytes().into();
        Ok(self.set_mime(mime::TEXT_PLAIN_UTF_8))
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
            let client = self.client.take().unwrap();
            let middleware = self.middleware.take().unwrap();
            let req = self.req.take().unwrap();

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

impl<R: AsyncRead + Unpin + Send + 'static> TryFrom<http::Request<Box<R>>>
    for Request<HyperClient>
{
    type Error = io::Error;

    /// Converts an `http::Request` to a `surf::Request`.
    fn try_from(http_request: http::Request<Box<R>>) -> io::Result<Self> {
        let (parts, body) = http_request.into_parts();
        let url = format!("{}", parts.uri);
        let req = Self::new(parts.method, url.parse().unwrap());
        let req = req.set_body(Box::new(Body::from(body)));
        Ok(req)
    }
}

impl<C: HttpClient> Into<http::Request<Body>> for Request<C> {
    /// Converts a `surf::Request` to an `http::Request`.
    fn into(self) -> http::Request<Body> {
        self.req.unwrap()
    }
}

impl<C: HttpClient> fmt::Debug for Request<C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.req, f)
    }
}
