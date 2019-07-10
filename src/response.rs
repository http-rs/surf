use futures::prelude::*;
use std::io;

use std::io::Error;
use std::pin::Pin;
use std::task::{Context, Poll};

use super::Fail;

/// A response returned by `surf::Client`.
#[derive(Debug)]
pub struct Response<R: AsyncRead> {
    response: hyper::Response<hyper::Body>,
    reader: R,
}

impl<R: AsyncRead + Unpin> Response<R> {
    /// Create a new instance.
    pub(crate) fn new(response: hyper::Response<hyper::Body>, reader: R) -> Self {
        Self { response, reader }
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
    pub async fn into_bytes(mut self) -> io::Result<Vec<u8>> {
        let mut buf = vec![];
        self.reader.read_to_end(&mut buf).await?;
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
    pub async fn into_string(self) -> Result<String, Fail> {
        let bytes = self.into_bytes().await?;
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
    pub async fn into_json<T: serde::de::DeserializeOwned>(self) -> std::io::Result<T> {
        let body_bytes = self.into_bytes().await?;
        Ok(serde_json::from_slice(&body_bytes).map_err(|_| std::io::ErrorKind::InvalidData)?)
    }
}

impl<R: AsyncRead> AsyncRead for Response<R> {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<Result<usize, Error>> {
        Pin::new(&mut self).poll_read(cx, buf)
    }
}
