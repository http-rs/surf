use futures::prelude::*;
use http::status::StatusCode;
use http::version::Version;
use mime::Mime;
use serde::de::DeserializeOwned;

use std::fmt;
use std::io::{self, Error};
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::headers::Headers;
use crate::http_client;
use crate::Exception;

/// An HTTP response, returned by `Request`.
pub struct Response {
    response: http_client::Response,
}

impl Response {
    /// Create a new instance.
    pub(crate) fn new(response: http_client::Response) -> Self {
        Self { response }
    }

    /// Get the HTTP status code.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # #![feature(async_await)]
    /// # #[runtime::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// let res = surf::get("https://httpbin.org/get").await?;
    /// assert_eq!(res.status(), 200);
    /// # Ok(()) }
    /// ```
    pub fn status(&self) -> StatusCode {
        self.response.status()
    }

    /// Get the HTTP protocol version.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # #![feature(async_await)]
    /// # #[runtime::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// use surf::http::version::Version;
    ///
    /// let res = surf::get("https://httpbin.org/get").await?;
    /// assert_eq!(res.version(), Version::HTTP_11);
    /// # Ok(()) }
    /// ```
    pub fn version(&self) -> Version {
        self.response.version()
    }

    /// Get a header.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # #![feature(async_await)]
    /// # #[runtime::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// let res = surf::get("https://httpbin.org/get").await?;
    /// assert!(res.header("Content-Length").is_some());
    /// # Ok(()) }
    /// ```
    pub fn header(&self, key: &'static str) -> Option<&'_ str> {
        let headers = self.response.headers();
        headers.get(key).map(|h| h.to_str().unwrap())
    }

    /// Get all headers.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # #![feature(async_await)]
    /// # #[runtime::main]
    /// # async fn main() -> Result<(), surf::Exception> {
    /// let mut res = surf::post("https://httpbin.org/get").await?;
    /// for (name, value) in res.headers() {
    ///     println!("{}: {}", name, value);
    /// }
    /// # Ok(()) }
    /// ```
    pub fn headers<'a>(&'a mut self) -> Headers<'a> {
        Headers::new(self.response.headers_mut())
    }

    /// Get the request MIME.
    ///
    /// Gets the `Content-Type` header and parses it to a `Mime` type.
    ///
    /// [Read more on MDN](https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types)
    ///
    /// # Panics
    ///
    /// This method will panic if an invalid MIME type was set as a header.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # #![feature(async_await)]
    /// # #[runtime::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// use surf::mime;
    /// let res = surf::get("https://httpbin.org/json").await?;
    /// assert_eq!(res.mime(), Some(mime::APPLICATION_JSON));
    /// # Ok(()) }
    /// ```
    pub fn mime(&self) -> Option<Mime> {
        let header = self.header("Content-Type")?;
        Some(header.parse().unwrap())
    }

    /// Reads the entire request body into a byte buffer.
    ///
    /// This method can be called after the body has already been read, but will
    /// produce an empty buffer.
    ///
    /// # Errors
    ///
    /// Any I/O error encountered while reading the body is immediately returned
    /// as an `Err`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # #![feature(async_await)]
    /// # #[runtime::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// let mut res = surf::get("https://httpbin.org/get").await?;
    /// let bytes: Vec<u8> = res.body_bytes().await?;
    /// # Ok(()) }
    /// ```
    pub async fn body_bytes(&mut self) -> io::Result<Vec<u8>> {
        let mut buf = Vec::with_capacity(1024);
        self.response.body_mut().read_to_end(&mut buf).await?;
        Ok(buf)
    }

    /// Reads the entire request body into a string.
    ///
    /// This method can be called after the body has already been read, but will
    /// produce an empty buffer.
    ///
    /// # Errors
    ///
    /// Any I/O error encountered while reading the body is immediately returned
    /// as an `Err`.
    ///
    /// If the body cannot be interpreted as valid UTF-8, an `Err` is returned.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # #![feature(async_await)]
    /// # #[runtime::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// let mut res = surf::get("https://httpbin.org/get").await?;
    /// let string: String = res.body_string().await?;
    /// # Ok(()) }
    /// ```
    pub async fn body_string(&mut self) -> Result<String, Exception> {
        let bytes = self.body_bytes().await?;
        Ok(String::from_utf8(bytes).map_err(|_| io::Error::from(io::ErrorKind::InvalidData))?)
    }

    /// Reads and deserialized the entire request body from json.
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
    /// # #![feature(async_await)]
    /// # use serde::{Deserialize, Serialize};
    /// # #[runtime::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// #[derive(Deserialize, Serialize)]
    /// struct Ip {
    ///     ip: String
    /// }
    ///
    /// let mut res = surf::get("https://api.ipify.org?format=json").await?;
    /// let Ip { ip } = res.body_json().await?;
    /// # Ok(()) }
    /// ```
    pub async fn body_json<T: DeserializeOwned>(&mut self) -> std::io::Result<T> {
        let body_bytes = self.body_bytes().await?;
        Ok(serde_json::from_slice(&body_bytes).map_err(|_| std::io::ErrorKind::InvalidData)?)
    }

    /// Reads and deserialized the entire request body from form encoding.
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
    /// # #![feature(async_await)]
    /// # use serde::{Deserialize, Serialize};
    /// # #[runtime::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// #[derive(Deserialize, Serialize)]
    /// struct Body {
    ///     apples: u32
    /// }
    ///
    /// let mut res = surf::get("https://api.example.com/v1/response").await?;
    /// let Body { apples } = res.body_form().await?;
    /// # Ok(()) }
    /// ```
    pub async fn body_form<T: serde::de::DeserializeOwned>(&mut self) -> Result<T, Exception> {
        use std::io::ErrorKind;
        let string = self.body_string().await?;
        Ok(serde_urlencoded::from_str(&string).map_err(|_| Error::from(ErrorKind::InvalidData))?)
    }
}

impl AsyncRead for Response {
    #[allow(missing_doc_code_examples)]
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<Result<usize, Error>> {
        Pin::new(&mut self.response.body_mut()).poll_read(cx, buf)
    }
}

impl fmt::Debug for Response {
    #[allow(missing_doc_code_examples)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Response")
            .field("response", &self.response)
            .finish()
    }
}
