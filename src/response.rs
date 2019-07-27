use futures::prelude::*;
use http::status::StatusCode;
use http::version::Version;
use serde::de::DeserializeOwned;

use std::fmt;
use std::io::{self, Error};
use std::pin::Pin;
use std::task::{Context, Poll};

use super::http_client;
use super::Exception;

/// A response returned by `Request`.
pub struct Response {
    response: http_client::Response,
}

impl Response {
    /// Create a new instance.
    pub(crate) fn new(response: http_client::Response) -> Self {
        Self { response }
    }

    /// Get the HTTP status code.
    pub fn status(&self) -> StatusCode {
        self.response.status()
    }

    /// Set the HTTP status code.
    pub fn set_status(&mut self, status: StatusCode) {
        *self.response.status_mut() = status;
    }

    /// Get the HTTP protocol version.
    pub fn version(&self) -> Version {
        self.response.version()
    }

    /// Set the HTTP protocol version.
    pub fn set_version(&mut self, version: Version) {
        *self.response.version_mut() = version;
    }

    /// Get a header.
    pub fn header(&self, key: &'static str) -> Option<&'_ str> {
        let headers = self.response.headers();
        headers.get(key).map(|h| h.to_str().unwrap())
    }

    /// Insert a header.
    pub fn set_header(mut self, key: &'static str, value: impl AsRef<str>) -> Self {
        let value = value.as_ref().to_owned();
        let headers = self.response.headers_mut();
        headers.insert(key, value.parse().unwrap());
        self
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
    pub async fn body_string(&mut self) -> Result<String, Exception> {
        let bytes = self.body_bytes().await?;
        Ok(String::from_utf8(bytes).map_err(|_| io::Error::from(io::ErrorKind::InvalidData))?)
    }

    /// Reads and deserialized the entire request body via json.
    ///
    /// # Errors
    ///
    /// Any I/O error encountered while reading the body is immediately returned
    /// as an `Err`.
    ///
    /// If the body cannot be interpreted as valid json for the target type `T`,
    /// an `Err` is returned.
    pub async fn body_json<T: DeserializeOwned>(&mut self) -> std::io::Result<T> {
        let body_bytes = self.body_bytes().await?;
        Ok(serde_json::from_slice(&body_bytes).map_err(|_| std::io::ErrorKind::InvalidData)?)
    }
}

impl AsyncRead for Response {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<Result<usize, Error>> {
        Pin::new(&mut self.response.body_mut()).poll_read(cx, buf)
    }
}

impl fmt::Debug for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Response")
            .field("response", &self.response)
            .finish()
    }
}
