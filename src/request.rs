use futures::future::BoxFuture;
use futures::prelude::*;
use http::Method;
use mime::Mime;
use serde::Serialize;
use url::Url;

use crate::headers::Headers;
use crate::http_client::{self, Body, HttpClient};
use crate::middleware::{Middleware, Next};
use crate::Exception;
use crate::Response;

use std::fmt;
use std::fmt::Debug;
use std::fs;
use std::future::Future;
use std::io;
use std::path::Path;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

#[cfg(feature = "native-client")]
use super::http_client::native::NativeClient;
#[cfg(feature = "native-client")]
use std::convert::TryFrom;

/// An HTTP request, returns a `Response`.
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

#[cfg(feature = "native-client")]
impl Request<NativeClient> {
    /// Create a new instance.
    ///
    /// This method is particularly useful when input URLs might be passed by third parties, and
    /// you don't want to panic if they're malformed. If URLs are statically encoded, it might be
    /// easier to use one of the shorthand methods instead.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # #[runtime::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// use surf::{http, url};
    ///
    /// let method = http::Method::GET;
    /// let url = url::Url::parse("https://httpbin.org/get")?;
    /// let string = surf::Request::new(method, url).recv_string().await?;
    /// # Ok(()) }
    /// ```
    pub fn new(method: http::Method, url: Url) -> Self {
        Self::with_client(method, url, NativeClient::new())
    }
}

impl<C: HttpClient> Request<C> {
    /// Create a new instance with an `HttpClient` instance.
    // TODO(yw): hidden from docs until we make the traits public.
    #[doc(hidden)]
    #[allow(missing_doc_code_examples)]
    pub fn with_client(method: http::Method, url: Url, client: C) -> Self {
        let mut req = http_client::Request::new(Body::empty());
        *req.method_mut() = method;
        *req.uri_mut() = url.as_str().parse().unwrap();
        let client = Self {
            fut: None,
            client: Some(client),
            req: Some(req),
            url,
            middleware: Some(vec![]),
        };

        #[cfg(feature = "middleware-logger")]
        let client = client.middleware(crate::middleware::logger::new());

        client
    }

    /// Push middleware onto the middleware stack.
    ///
    /// See the [middleware] submodule for more information on middleware.
    ///
    /// [middleware]: ../middleware/index.html
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # #[runtime::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// let res = surf::get("https://httpbin.org/get")
    ///     .middleware(surf::middleware::logger::new())
    ///     .await?;
    /// # Ok(()) }
    /// ```
    pub fn middleware(mut self, mw: impl Middleware<C>) -> Self {
        self.middleware.as_mut().unwrap().push(Arc::new(mw));
        self
    }

    /// Get the URL querystring.
    ///
    /// # Examples
    ///
    /// ```
    /// # use serde::{Deserialize, Serialize};
    /// # #[runtime::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// #[derive(Serialize, Deserialize)]
    /// struct Index {
    ///     page: u32
    /// }
    ///
    /// let req = surf::get("https://httpbin.org/get?page=2");
    /// let Index { page } = req.query()?;
    /// assert_eq!(page, 2);
    /// # Ok(()) }
    /// ```
    pub fn query<T: serde::de::DeserializeOwned>(&self) -> Result<T, Exception> {
        use std::io::{Error, ErrorKind};
        let query = self
            .url
            .query()
            .ok_or_else(|| Error::from(ErrorKind::InvalidData))?;
        Ok(serde_urlencoded::from_str(query)?)
    }

    /// Set the URL querystring.
    ///
    /// # Examples
    ///
    /// ```
    /// # use serde::{Deserialize, Serialize};
    /// # #[runtime::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// #[derive(Serialize, Deserialize)]
    /// struct Index {
    ///     page: u32
    /// }
    ///
    /// let query = Index { page: 2 };
    /// let req = surf::get("https://httpbin.org/get").set_query(&query)?;
    /// assert_eq!(req.url().query(), Some("page=2"));
    /// # Ok(()) }
    /// ```
    pub fn set_query(
        mut self,
        query: &impl Serialize,
    ) -> Result<Self, serde_urlencoded::ser::Error> {
        let query = serde_urlencoded::to_string(query)?;
        self.url.set_query(Some(&query));
        Ok(self)
    }

    /// Get an HTTP header.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[runtime::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// let req = surf::get("https://httpbin.org/get")
    ///     .set_header("X-Requested-With", "surf");
    /// assert_eq!(req.header("X-Requested-With"), Some("surf"));
    /// # Ok(()) }
    /// ```
    pub fn header(&self, key: &'static str) -> Option<&'_ str> {
        let req = self.req.as_ref().unwrap();
        req.headers().get(key).map(|h| h.to_str().unwrap())
    }

    /// Set an HTTP header.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[runtime::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// let req = surf::get("https://httpbin.org/get")
    ///     .set_header("X-Requested-With", "surf");
    /// assert_eq!(req.header("X-Requested-With"), Some("surf"));
    /// # Ok(()) }
    /// ```
    pub fn set_header(mut self, key: &'static str, value: impl AsRef<str>) -> Self {
        let value = value.as_ref().to_owned();
        let req = self.req.as_mut().unwrap();
        req.headers_mut().insert(key, value.parse().unwrap());
        self
    }

    /// Get all headers.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # #[runtime::main]
    /// # async fn main() -> Result<(), surf::Exception> {
    /// let mut req = surf::get("https://httpbin.org/get")
    ///     .set_header("X-Requested-With", "surf");
    ///
    /// for (name, value) in req.headers() {
    ///     println!("{}: {}", name, value);
    /// }
    /// # Ok(()) }
    /// ```
    pub fn headers(&mut self) -> Headers<'_> {
        Headers::new(self.req.as_mut().unwrap().headers_mut())
    }

    /// Get the request HTTP method.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[runtime::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// use surf::http;
    /// let req = surf::get("https://httpbin.org/get");
    /// assert_eq!(req.method(), http::Method::GET);
    /// # Ok(()) }
    /// ```
    pub fn method(&self) -> &Method {
        let req = self.req.as_ref().unwrap();
        req.method()
    }

    /// Get the request url.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[runtime::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// use surf::url::Url;
    /// let req = surf::get("https://httpbin.org/get");
    /// assert_eq!(req.url(), &Url::parse("https://httpbin.org/get")?);
    /// # Ok(()) }
    /// ```
    pub fn url(&self) -> &Url {
        &self.url
    }

    /// Get the request MIME.
    ///
    /// Gets the `Content-Type` header and parses it to a `Mime` type.
    ///
    /// [Read more on MDN](https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types)
    ///
    /// # Panics
    ///
    /// This method will panic if an invalid MIME type was set as a header. Use the [`set_header`]
    /// method to bypass any checks.
    ///
    /// [`set_header`]: #method.set_header
    ///
    /// # Examples
    ///
    /// ```
    /// # #[runtime::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// use surf::mime;
    /// let req = surf::post("https://httpbin.org/get")
    ///     .set_mime(mime::TEXT_CSS);
    /// assert_eq!(req.mime(), Some(mime::TEXT_CSS));
    /// # Ok(()) }
    /// ```
    pub fn mime(&self) -> Option<Mime> {
        let header = self.header("Content-Type")?;
        Some(header.parse().unwrap())
    }

    /// Set the request MIME.
    ///
    /// [Read more on MDN](https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types)
    ///
    /// # Examples
    ///
    /// ```
    /// # #[runtime::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// use surf::mime;
    /// let req = surf::post("https://httpbin.org/get")
    ///     .set_mime(mime::TEXT_CSS);
    /// assert_eq!(req.mime(), Some(mime::TEXT_CSS));
    /// # Ok(()) }
    /// ```
    pub fn set_mime(self, mime: Mime) -> Self {
        self.set_header("Content-Type", format!("{}", mime))
    }

    /// Pass an `AsyncRead` stream as the request body.
    ///
    /// # Mime
    ///
    /// The encoding is set to `application/octet-stream`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # #[runtime::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// let reader = surf::get("https://httpbin.org/get").await?;
    /// let uri = "https://httpbin.org/post";
    /// let res = surf::post(uri).body(reader).await?;
    /// assert_eq!(res.status(), 200);
    /// # Ok(()) }
    /// ```
    pub fn body<R>(mut self, reader: R) -> Self
    where
        R: AsyncRead + Unpin + Send + 'static,
    {
        *self.req.as_mut().unwrap().body_mut() = Box::new(reader).into();
        self.set_mime(mime::APPLICATION_OCTET_STREAM)
    }

    /// Pass JSON as the request body.
    ///
    /// # Mime
    ///
    /// The encoding is set to `application/json`.
    ///
    /// # Errors
    ///
    /// This method will return an error if the provided data could not be serialized to JSON.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # #[runtime::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// let uri = "https://httpbin.org/post";
    /// let data = serde_json::json!({ "name": "chashu" });
    /// let res = surf::post(uri).body_json(&data)?.await?;
    /// assert_eq!(res.status(), 200);
    /// # Ok(()) }
    /// ```
    pub fn body_json(mut self, json: &impl Serialize) -> serde_json::Result<Self> {
        *self.req.as_mut().unwrap().body_mut() = serde_json::to_vec(json)?.into();
        Ok(self.set_mime(mime::APPLICATION_JSON))
    }

    /// Pass a string as the request body.
    ///
    /// # Mime
    ///
    /// The encoding is set to `text/plain; charset=utf-8`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # #[runtime::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// let uri = "https://httpbin.org/post";
    /// let data = "hello world".to_string();
    /// let res = surf::post(uri).body_string(data).await?;
    /// assert_eq!(res.status(), 200);
    /// # Ok(()) }
    /// ```
    pub fn body_string(mut self, string: String) -> Self {
        *self.req.as_mut().unwrap().body_mut() = string.into_bytes().into();
        self.set_mime(mime::TEXT_PLAIN_UTF_8)
    }

    /// Pass bytes as the request body.
    ///
    /// # Mime
    ///
    /// The encoding is set to `application/octet-stream`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # #[runtime::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// let uri = "https://httpbin.org/post";
    /// let data = b"hello world";
    /// let res = surf::post(uri).body_bytes(data).await?;
    /// assert_eq!(res.status(), 200);
    /// # Ok(()) }
    /// ```
    pub fn body_bytes(mut self, bytes: impl AsRef<[u8]>) -> Self {
        *self.req.as_mut().unwrap().body_mut() = bytes.as_ref().to_owned().into();
        self.set_mime(mime::APPLICATION_OCTET_STREAM)
    }

    /// Pass a file as the request body.
    ///
    /// # Mime
    ///
    /// The encoding is set based on the file extension using [`mime_guess`] if the operation was
    /// successful. If `path` has no extension, or its extension has no known MIME type mapping,
    /// then `None` is returned.
    ///
    /// [`mime_guess`]: https://docs.rs/mime_guess
    ///
    /// # Errors
    ///
    /// This method will return an error if the file couldn't be read.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # #[runtime::main]
    /// # async fn main() -> Result<(), surf::Exception> {
    /// let res = surf::post("https://httpbin.org/post")
    ///     .body_file("README.md")?
    ///     .await?;
    /// assert_eq!(res.status(), 200);
    /// # Ok(()) }
    /// ```
    pub fn body_file(mut self, path: impl AsRef<Path>) -> io::Result<Self> {
        let mime = mime_guess::from_path(&path).first_or_octet_stream();
        let bytes = fs::read(path)?;
        *self.req.as_mut().unwrap().body_mut() = bytes.into();
        Ok(self.set_mime(mime))
    }

    /// Pass a form as the request body.
    ///
    /// # Mime
    ///
    /// The encoding is set to `application/x-www-form-urlencoded`.
    ///
    /// # Errors
    ///
    /// An error will be returned if the encoding failed.
    ///
    /// # Examples
    ///
    /// ```
    /// # use serde::{Deserialize, Serialize};
    /// # #[runtime::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// #[derive(Serialize, Deserialize)]
    /// struct Body {
    ///     apples: u32
    /// }
    ///
    /// let res = surf::post("https://httpbin.org/post")
    ///     .body_form(&Body { apples: 7 })?
    ///     .await?;
    /// assert_eq!(res.status(), 200);
    /// # Ok(()) }
    /// ```
    pub fn body_form(
        mut self,
        form: &impl Serialize,
    ) -> Result<Self, serde_urlencoded::ser::Error> {
        let query = serde_urlencoded::to_string(form)?;
        self = self.body_string(query);
        self = self.set_mime(mime::APPLICATION_WWW_FORM_URLENCODED);
        Ok(self)
    }

    /// Submit the request and get the response body as bytes.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # #[runtime::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// let bytes = surf::get("https://httpbin.org/get").recv_bytes().await?;
    /// assert!(bytes.len() > 0);
    /// # Ok(()) }
    /// ```
    pub async fn recv_bytes(self) -> Result<Vec<u8>, Exception> {
        let mut req = self.await?;
        Ok(req.body_bytes().await?)
    }

    /// Submit the request and get the response body as a string.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # #[runtime::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// let string = surf::get("https://httpbin.org/get").recv_string().await?;
    /// assert!(string.len() > 0);
    /// # Ok(()) }
    /// ```
    pub async fn recv_string(self) -> Result<String, Exception> {
        let mut req = self.await?;
        Ok(req.body_string().await?)
    }

    /// Submit the request and decode the response body from json into a struct.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use serde::{Deserialize, Serialize};
    /// # #[runtime::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// #[derive(Deserialize, Serialize)]
    /// struct Ip {
    ///     ip: String
    /// }
    ///
    /// let uri = "https://api.ipify.org?format=json";
    /// let Ip { ip } = surf::get(uri).recv_json().await?;
    /// assert!(ip.len() > 10);
    /// # Ok(()) }
    /// ```
    pub async fn recv_json<T: serde::de::DeserializeOwned>(self) -> Result<T, Exception> {
        let mut req = self.await?;
        Ok(req.body_json::<T>().await?)
    }

    /// Submit the request and decode the response body from form encoding into a struct.
    ///
    /// # Errors
    ///
    /// Any I/O error encountered while reading the body is immediately returned
    /// as an `Err`.
    ///
    /// If the body cannot be interpreted as valid json for the target type `T`,
    /// an `Err` is returned.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use serde::{Deserialize, Serialize};
    /// # #[runtime::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// #[derive(Deserialize, Serialize)]
    /// struct Body {
    ///     apples: u32
    /// }
    ///
    /// let url = "https://api.example.com/v1/response";
    /// let Body { apples } = surf::get(url).recv_form().await?;
    /// # Ok(()) }
    /// ```
    pub async fn recv_form<T: serde::de::DeserializeOwned>(self) -> Result<T, Exception> {
        let mut req = self.await?;
        Ok(req.body_form::<T>().await?)
    }
}

impl<C: HttpClient> Future for Request<C> {
    type Output = Result<Response, Exception>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.fut.is_none() {
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

#[cfg(feature = "native-client")]
impl<R: AsyncRead + Unpin + Send + 'static> TryFrom<http::Request<Box<R>>>
    for Request<NativeClient>
{
    type Error = io::Error;

    /// Converts an `http::Request` to a `surf::Request`.
    fn try_from(http_request: http::Request<Box<R>>) -> io::Result<Self> {
        let (parts, body) = http_request.into_parts();
        let url = format!("{}", parts.uri);
        let req = Self::new(parts.method, url.parse().unwrap());
        let req = req.body(Box::new(Body::from(body)));
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
