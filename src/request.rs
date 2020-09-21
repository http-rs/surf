use crate::http::{
    self,
    headers::{self, HeaderName, HeaderValues, ToHeaderValues},
    Body, Error, Method, Mime,
};
use crate::RequestBuilder;

use serde::Serialize;
use url::Url;

use std::fmt;
use std::ops::Index;

/// An HTTP request, returns a `Response`.
#[derive(Clone)]
pub struct Request {
    /// Holds the state of the request.
    req: http_client::Request,
}

impl Request {
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
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// use surf::http::Method;
    /// use surf::url::Url;
    ///
    /// let method = Method::Get;
    /// let url = Url::parse("https://httpbin.org/get")?;
    /// let req = surf::Request::new(method, url);
    /// # Ok(()) }
    /// ```
    pub fn new(method: Method, url: Url) -> Self {
        let req = http_client::Request::new(method, url);
        Self { req }
    }

    /// Begin a chained request builder. For more details, see [RequestBuilder](crate::RequestBuilder)
    ///
    /// # Example:
    /// ```rust
    /// # use surf::url::Url;
    /// # use surf::{http, Request};
    /// # #[async_std::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// let url = Url::parse("https://httpbin.org/post")?;
    /// let mut request = Request::builder(http::Method::Post, url.clone())
    ///     .body("<html>hi</html>")
    ///     .header("custom-header", "value")
    ///     .content_type(http::mime::HTML)
    ///     .build();
    ///
    /// assert_eq!(request.take_body().into_string().await.unwrap(), "<html>hi</html>");
    /// assert_eq!(request.method(), http::Method::Post);
    /// assert_eq!(request.url(), &url);
    /// assert_eq!(request["custom-header"], "value");
    /// assert_eq!(request["content-type"], "text/html;charset=utf-8");
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn builder(method: Method, url: Url) -> RequestBuilder {
        RequestBuilder::new(method, url)
    }

    /// Get the URL querystring.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use serde::{Deserialize, Serialize};
    /// # #[async_std::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// #[derive(Serialize, Deserialize)]
    /// struct Index {
    ///     page: u32
    /// }
    ///
    /// let req = surf::get("https://httpbin.org/get?page=2").build();
    /// let Index { page } = req.query()?;
    /// assert_eq!(page, 2);
    /// # Ok(()) }
    /// ```
    pub fn query<T: serde::de::DeserializeOwned>(&self) -> Result<T, Error> {
        use std::io::{Error, ErrorKind};
        let query = self
            .req
            .url()
            .query()
            .ok_or_else(|| Error::from(ErrorKind::InvalidData))?;
        Ok(serde_urlencoded::from_str(query)?)
    }

    /// Set the URL querystring.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use serde::{Deserialize, Serialize};
    /// # #[async_std::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// #[derive(Serialize, Deserialize)]
    /// struct Index {
    ///     page: u32
    /// }
    ///
    /// let query = Index { page: 2 };
    /// let mut req = surf::get("https://httpbin.org/get").build();
    /// req.set_query(&query)?;
    /// assert_eq!(req.url().query(), Some("page=2"));
    /// assert_eq!(req.as_ref().url().as_str(), "https://httpbin.org/get?page=2");
    /// # Ok(()) }
    /// ```
    pub fn set_query(
        &mut self,
        query: &(impl Serialize + ?Sized),
    ) -> Result<(), serde_urlencoded::ser::Error> {
        let query = serde_urlencoded::to_string(query)?;
        self.req.url_mut().set_query(Some(&query));

        *self.req.url_mut() = self.req.url().clone();

        Ok(())
    }

    /// Get an HTTP header.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # #[async_std::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// let mut req = surf::get("https://httpbin.org/get").build();
    /// req.set_header("X-Requested-With", "surf");
    /// assert_eq!(req.header("X-Requested-With").unwrap(), "surf");
    /// # Ok(()) }
    /// ```
    pub fn header(&self, key: impl Into<HeaderName>) -> Option<&HeaderValues> {
        self.req.header(key)
    }

    /// Get a mutable reference to a header.
    pub fn header_mut(&mut self, name: impl Into<HeaderName>) -> Option<&mut HeaderValues> {
        self.req.header_mut(name)
    }

    /// Set an HTTP header.
    pub fn insert_header(
        &mut self,
        name: impl Into<HeaderName>,
        values: impl ToHeaderValues,
    ) -> Option<HeaderValues> {
        self.req.insert_header(name, values)
    }

    /// Append a header to the headers.
    ///
    /// Unlike `insert` this function will not override the contents of a header, but insert a
    /// header if there aren't any. Or else append to the existing list of headers.
    pub fn append_header(&mut self, name: impl Into<HeaderName>, values: impl ToHeaderValues) {
        self.req.append_header(name, values)
    }

    /// Remove a header.
    pub fn remove_header(&mut self, name: impl Into<HeaderName>) -> Option<HeaderValues> {
        self.req.remove_header(name)
    }

    /// An iterator visiting all header pairs in arbitrary order.
    #[must_use]
    pub fn iter(&self) -> headers::Iter<'_> {
        self.req.iter()
    }

    /// An iterator visiting all header pairs in arbitrary order, with mutable references to the
    /// values.
    #[must_use]
    pub fn iter_mut(&mut self) -> headers::IterMut<'_> {
        self.req.iter_mut()
    }

    /// An iterator visiting all header names in arbitrary order.
    #[must_use]
    pub fn header_names(&self) -> headers::Names<'_> {
        self.req.header_names()
    }

    /// An iterator visiting all header values in arbitrary order.
    #[must_use]
    pub fn header_values(&self) -> headers::Values<'_> {
        self.req.header_values()
    }

    /// Set an HTTP header.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # #[async_std::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// let mut req = surf::get("https://httpbin.org/get").build();
    /// req.set_header("X-Requested-With", "surf");
    /// assert_eq!(req.header("X-Requested-With").unwrap(), "surf");
    /// # Ok(()) }
    /// ```
    pub fn set_header(&mut self, key: impl Into<HeaderName>, value: impl ToHeaderValues) {
        self.insert_header(key, value);
    }

    /// Get a request extension value.
    #[must_use]
    pub fn ext<T: Send + Sync + 'static>(&self) -> Option<&T> {
        self.req.ext().get()
    }

    /// Set a request extension value.
    pub fn set_ext<T: Send + Sync + 'static>(&mut self, val: T) -> Option<T> {
        self.req.ext_mut().insert(val)
    }

    /// Get the request HTTP method.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # #[async_std::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// let req = surf::get("https://httpbin.org/get").build();
    /// assert_eq!(req.method(), surf::http::Method::Get);
    /// # Ok(()) }
    /// ```
    pub fn method(&self) -> Method {
        self.req.method()
    }

    /// Get the request url.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # #[async_std::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// use surf::url::Url;
    /// let req = surf::get("https://httpbin.org/get").build();
    /// assert_eq!(req.url(), &Url::parse("https://httpbin.org/get")?);
    /// # Ok(()) }
    /// ```
    pub fn url(&self) -> &Url {
        self.req.url()
    }

    /// Get the request content type as a `Mime`.
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
    pub fn content_type(&self) -> Option<Mime> {
        self.req.content_type()
    }

    /// Set the request content type from a `Mime`.
    ///
    /// [Read more on MDN](https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types)
    pub fn set_content_type(&mut self, mime: Mime) {
        self.req.set_content_type(mime);
    }

    /// Get the length of the body stream, if it has been set.
    ///
    /// This value is set when passing a fixed-size object into as the body.
    /// E.g. a string, or a buffer. Consumers of this API should check this
    /// value to decide whether to use `Chunked` encoding, or set the
    /// response length.
    pub fn len(&self) -> Option<usize> {
        self.req.len()
    }

    /// Returns `true` if the set length of the body stream is zero, `false`
    /// otherwise.
    pub fn is_empty(&self) -> Option<bool> {
        self.req.is_empty()
    }

    /// Pass an `AsyncRead` stream as the request body.
    ///
    /// # Mime
    ///
    /// The encoding is set to `application/octet-stream`.
    pub fn set_body(&mut self, body: impl Into<Body>) {
        self.req.set_body(body)
    }

    /// Take the request body as a `Body`.
    ///
    /// This method can be called after the body has already been taken or read,
    /// but will return an empty `Body`.
    ///
    /// This is useful for consuming the body via an AsyncReader or AsyncBufReader.
    pub fn take_body(&mut self) -> Body {
        self.req.take_body()
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
    pub fn body_json(&mut self, json: &impl Serialize) -> crate::Result<()> {
        self.set_body(Body::from_json(json)?);
        Ok(())
    }

    /// Pass a string as the request body.
    ///
    /// # Mime
    ///
    /// The encoding is set to `text/plain; charset=utf-8`.
    pub fn body_string(&mut self, string: String) {
        self.set_body(Body::from_string(string))
    }

    /// Pass bytes as the request body.
    ///
    /// # Mime
    ///
    /// The encoding is set to `application/octet-stream`.
    pub fn body_bytes(&mut self, bytes: impl AsRef<[u8]>) {
        self.set_body(Body::from(bytes.as_ref()))
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
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn body_file(&mut self, path: impl AsRef<std::path::Path>) -> std::io::Result<()> {
        self.set_body(Body::from_file(path).await?);
        Ok(())
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
    pub fn body_form(&mut self, form: &impl Serialize) -> crate::Result<()> {
        self.set_body(Body::from_form(form)?);
        Ok(())
    }
}

impl AsRef<http::Request> for Request {
    fn as_ref(&self) -> &http::Request {
        &self.req
    }
}

impl AsMut<http::Request> for Request {
    fn as_mut(&mut self) -> &mut http::Request {
        &mut self.req
    }
}

impl From<http::Request> for Request {
    /// Converts an `http::Request` to a `surf::Request`.
    fn from(req: http::Request) -> Self {
        Self { req }
    }
}

impl Into<http::Request> for Request {
    /// Converts a `surf::Request` to an `http::Request`.
    fn into(self) -> http::Request {
        self.req
    }
}

impl fmt::Debug for Request {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.req, f)
    }
}

impl IntoIterator for Request {
    type Item = (HeaderName, HeaderValues);
    type IntoIter = headers::IntoIter;

    /// Returns a iterator of references over the remaining items.
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.req.into_iter()
    }
}

impl<'a> IntoIterator for &'a Request {
    type Item = (&'a HeaderName, &'a HeaderValues);
    type IntoIter = headers::Iter<'a>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.req.iter()
    }
}

impl<'a> IntoIterator for &'a mut Request {
    type Item = (&'a HeaderName, &'a mut HeaderValues);
    type IntoIter = headers::IterMut<'a>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.req.iter_mut()
    }
}

impl Index<HeaderName> for Request {
    type Output = HeaderValues;

    /// Returns a reference to the value corresponding to the supplied name.
    ///
    /// # Panics
    ///
    /// Panics if the name is not present in `Request`.
    #[inline]
    fn index(&self, name: HeaderName) -> &HeaderValues {
        &self.req[name]
    }
}

impl Index<&str> for Request {
    type Output = HeaderValues;

    /// Returns a reference to the value corresponding to the supplied name.
    ///
    /// # Panics
    ///
    /// Panics if the name is not present in `Request`.
    #[inline]
    fn index(&self, name: &str) -> &HeaderValues {
        &self.req[name]
    }
}
