use futures::compat::Compat01As03;
use futures::prelude::*;
use std::io;

use super::Fail;

/// A response returned by `surf::Client`.
#[derive(Debug)]
pub struct Response {
    response: hyper::Response<hyper::Body>,
}

impl Response {
    /// Create a new instance
    fn new(response: hyper::Response<hyper::Body>) -> Self {
        Self { response }
    }

    /// Remove ownership of the request body, replacing it with an empty body.
    ///
    /// Used primarily for working directly with the body stream.
    fn take_body(&mut self) -> hyper::Body {
        std::mem::replace(self.response.body_mut(), hyper::Body::empty())
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
    pub async fn into_bytes(&mut self) -> Result<Vec<u8>, Fail> {
        let mut body = Compat01As03::new(self.take_body());
        let mut bytes = Vec::new();
        while let Some(chunk) = body.next().await {
            bytes.extend(chunk?);
        }
        Ok(bytes)
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
    pub async fn into_string(&mut self) -> Result<String, Fail> {
        let bytes = self.into_bytes().await?;
        Ok(String::from_utf8(bytes).map_err(|_| io::Error::from(io::ErrorKind::InvalidData))?)
    }
}
