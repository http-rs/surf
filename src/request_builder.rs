use crate::http::{
    headers::{HeaderName, ToHeaderValues},
    Body, Method, Mime, Url,
};
use crate::middleware::Middleware;
use crate::{Client, Error, Request, Response, Result};

use futures_util::future::BoxFuture;
use serde::Serialize;

use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

/// Request Builder
///
/// Provides an ergonomic way to chain the creation of a request.
/// This is generally accessed as the return value from `surf::{method}()`,
/// however [`Request::builder`](crate::Request::builder) is also provided.
///
/// # Examples
///
/// ```rust
/// use surf::http::{Method, mime::HTML, Url};
/// # #[async_std::main]
/// # async fn main() -> surf::Result<()> {
/// let mut request = surf::post("https://httpbin.org/post")
///     .body("<html>hi</html>")
///     .header("custom-header", "value")
///     .content_type(HTML)
///     .build();
///
/// assert_eq!(request.take_body().into_string().await.unwrap(), "<html>hi</html>");
/// assert_eq!(request.method(), Method::Post);
/// assert_eq!(request.url(), &Url::parse("https://httpbin.org/post")?);
/// assert_eq!(request["custom-header"], "value");
/// assert_eq!(request["content-type"], "text/html;charset=utf-8");
/// # Ok(())
/// # }
/// ```
///
/// ```rust
/// use surf::http::{Method, Url};
/// # #[async_std::main]
/// # async fn main() -> surf::Result<()> {
/// let url = Url::parse("https://httpbin.org/post")?;
/// let request = surf::Request::builder(Method::Post, url).build();
/// # Ok(())
/// # }
/// ```

pub struct RequestBuilder {
    /// Holds the state of the request.
    req: Option<Request>,
    /// Hold an optional Client.
    client: Option<Client>,
    /// Holds the state of the `impl Future`.
    fut: Option<BoxFuture<'static, Result<Response>>>,
}

impl RequestBuilder {
    /// Create a new instance.
    ///
    /// This method is particularly useful when input URLs might be passed by third parties, and
    /// you don't want to panic if they're malformed. If URLs are statically encoded, it might be
    /// easier to use one of the shorthand methods instead.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # #[async_std::main]
    /// # async fn main() -> surf::Result<()> {
    /// use surf::http::{Method, Url};
    ///
    /// let url = Url::parse("https://httpbin.org/get")?;
    /// let req = surf::RequestBuilder::new(Method::Get, url).build();
    /// # Ok(()) }
    /// ```
    pub fn new(method: Method, url: Url) -> Self {
        Self {
            req: Some(Request::new(method, url)),
            client: None,
            fut: None,
        }
    }

    pub(crate) fn with_client(mut self, client: Client) -> Self {
        let req = self.req.as_mut().unwrap();

        for (header_name, header_values) in client.config().headers.iter() {
            req.append_header(header_name, header_values);
        }

        self.client = Some(client);
        self
    }

    /// Sets a header on the request.
    ///
    /// # Examples
    ///
    /// ```
    /// let req = surf::get("https://httpbin.org/get").header("header-name", "header-value").build();
    /// assert_eq!(req["header-name"], "header-value");
    /// ```
    pub fn header(mut self, key: impl Into<HeaderName>, value: impl ToHeaderValues) -> Self {
        self.req.as_mut().unwrap().insert_header(key, value);
        self
    }

    /// Sets the Content-Type header on the request.
    ///
    /// # Examples
    ///
    /// ```
    /// # use surf::http::mime;
    /// let req = surf::post("https://httpbin.org/post").content_type(mime::HTML).build();
    /// assert_eq!(req["content-type"], "text/html;charset=utf-8");
    /// ```
    pub fn content_type(mut self, content_type: impl Into<Mime>) -> Self {
        self.req
            .as_mut()
            .unwrap()
            .set_content_type(content_type.into());
        self
    }

    /// Sets the body of the request from any type with implements `Into<Body>`, for example, any type with is `AsyncRead`.
    /// # Mime
    ///
    /// The encoding is set to `application/octet-stream`.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[async_std::main]
    /// # async fn main() -> surf::Result<()> {
    /// use serde_json::json;
    /// let mut req = surf::post("https://httpbin.org/post").body(json!({ "any": "Into<Body>"})).build();
    /// assert_eq!(req.take_body().into_string().await.unwrap(), "{\"any\":\"Into<Body>\"}");
    /// # Ok(())
    /// # }
    /// ```
    pub fn body(mut self, body: impl Into<Body>) -> Self {
        self.req.as_mut().unwrap().set_body(body);
        self
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
    /// # use serde::{Deserialize, Serialize};
    /// # #[async_std::main]
    /// # async fn main() -> surf::Result<()> {
    /// #[derive(Deserialize, Serialize)]
    /// struct Ip {
    ///     ip: String
    /// }
    ///
    /// let uri = "https://httpbin.org/post";
    /// let data = &Ip { ip: "129.0.0.1".into() };
    /// let res = surf::post(uri).body_json(data)?.await?;
    /// assert_eq!(res.status(), 200);
    /// # Ok(()) }
    /// ```
    pub fn body_json(self, json: &impl Serialize) -> crate::Result<Self> {
        Ok(self.body(Body::from_json(json)?))
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
    /// # #[async_std::main]
    /// # async fn main() -> surf::Result<()> {
    /// let uri = "https://httpbin.org/post";
    /// let data = "hello world".to_string();
    /// let res = surf::post(uri).body_string(data).await?;
    /// assert_eq!(res.status(), 200);
    /// # Ok(()) }
    /// ```
    pub fn body_string(self, string: String) -> Self {
        self.body(Body::from_string(string))
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
    /// # #[async_std::main]
    /// # async fn main() -> surf::Result<()> {
    /// let uri = "https://httpbin.org/post";
    /// let data = b"hello world".to_owned();
    /// let res = surf::post(uri).body_bytes(data).await?;
    /// assert_eq!(res.status(), 200);
    /// # Ok(()) }
    /// ```
    pub fn body_bytes(self, bytes: impl AsRef<[u8]>) -> Self {
        self.body(Body::from(bytes.as_ref()))
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
    /// # #[async_std::main]
    /// # async fn main() -> surf::Result<()> {
    /// let uri = "https://httpbin.org/post";
    /// let res = surf::post(uri).body_file("./archive.tgz").await?.await?;
    /// assert_eq!(res.status(), 200);
    /// # Ok(()) }
    /// ```
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn body_file(self, path: impl AsRef<std::path::Path>) -> std::io::Result<Self> {
        Ok(self.body(Body::from_file(path).await?))
    }

    /// Set the URL querystring.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use serde::{Deserialize, Serialize};
    /// # #[async_std::main]
    /// # async fn main() -> surf::Result<()> {
    /// #[derive(Serialize, Deserialize)]
    /// struct Index {
    ///     page: u32
    /// }
    ///
    /// let query = Index { page: 2 };
    /// let mut req = surf::get("https://httpbin.org/get").query(&query)?.build();
    /// assert_eq!(req.url().query(), Some("page=2"));
    /// assert_eq!(req.url().as_str(), "https://httpbin.org/get?page=2");
    /// # Ok(()) }
    /// ```
    pub fn query(mut self, query: &impl Serialize) -> std::result::Result<Self, Error> {
        self.req.as_mut().unwrap().set_query(query)?;

        Ok(self)
    }

    /// Submit the request and get the response body as bytes.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # #[async_std::main]
    /// # async fn main() -> surf::Result<()> {
    /// let bytes = surf::get("https://httpbin.org/get").recv_bytes().await?;
    /// assert!(bytes.len() > 0);
    /// # Ok(()) }
    /// ```
    pub async fn recv_bytes(self) -> Result<Vec<u8>> {
        let mut res = self.send().await?;
        Ok(res.body_bytes().await?)
    }

    /// Submit the request and get the response body as a string.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # #[async_std::main]
    /// # async fn main() -> surf::Result<()> {
    /// let string = surf::get("https://httpbin.org/get").recv_string().await?;
    /// assert!(string.len() > 0);
    /// # Ok(()) }
    /// ```
    pub async fn recv_string(self) -> Result<String> {
        let mut res = self.send().await?;
        Ok(res.body_string().await?)
    }

    /// Submit the request and decode the response body from json into a struct.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use serde::{Deserialize, Serialize};
    /// # #[async_std::main]
    /// # async fn main() -> surf::Result<()> {
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
    pub async fn recv_json<T: serde::de::DeserializeOwned>(self) -> Result<T> {
        let mut res = self.send().await?;
        Ok(res.body_json::<T>().await?)
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
    /// # #[async_std::main]
    /// # async fn main() -> surf::Result<()> {
    /// #[derive(Deserialize, Serialize)]
    /// struct Body {
    ///     apples: u32
    /// }
    ///
    /// let url = "https://api.example.com/v1/response";
    /// let Body { apples } = surf::get(url).recv_form().await?;
    /// # Ok(()) }
    /// ```
    pub async fn recv_form<T: serde::de::DeserializeOwned>(self) -> Result<T> {
        let mut res = self.send().await?;
        Ok(res.body_form::<T>().await?)
    }

    /// Push middleware onto a per-request middleware stack.
    ///
    /// **Important**: Setting per-request middleware incurs extra allocations.
    /// Creating a `Client` with middleware is recommended.
    ///
    /// Client middleware is run before per-request middleware.
    ///
    /// See the [middleware] submodule for more information on middleware.
    ///
    /// [middleware]: ../middleware/index.html
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # #[async_std::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// let res = surf::get("https://httpbin.org/get")
    ///     .middleware(surf::middleware::Redirect::default())
    ///     .await?;
    /// # Ok(()) }
    /// ```
    pub fn middleware(mut self, middleware: impl Middleware) -> Self {
        self.req.as_mut().unwrap().middleware(middleware);
        self
    }

    /// Return the constructed `Request`.
    pub fn build(self) -> Request {
        self.req.unwrap()
    }

    /// Create a `Client` and send the constructed `Request` from it.
    pub async fn send(mut self) -> Result<Response> {
        self.client
            .take()
            .unwrap_or_else(Client::new_shared_or_panic)
            .send(self.build())
            .await
    }
}

impl fmt::Debug for RequestBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.req, f)
    }
}

impl Future for RequestBuilder {
    type Output = Result<Response>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.fut.is_none() {
            let req = self.req.take().unwrap();

            let client = self
                .client
                .take()
                .unwrap_or_else(Client::new_shared_or_panic);

            self.fut = Some(Box::pin(async move { client.send(req).await }))
        }

        // We can safely unwrap here because this is the only time we take ownership of the request.
        self.fut.as_mut().unwrap().as_mut().poll(cx)
    }
}

impl From<RequestBuilder> for Request {
    /// Converts a `surf::RequestBuilder` to a `surf::Request`.
    fn from(builder: RequestBuilder) -> Request {
        builder.build()
    }
}
