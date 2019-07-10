//! HTTP client framework.
//!
//! ## Example
//!
//! ```rust
//! ```

#![forbid(unsafe_code, future_incompatible, rust_2018_idioms)]
#![deny(missing_debug_implementations, nonstandard_style)]
#![warn(missing_docs, missing_doc_code_examples, unreachable_pub)]
#![cfg_attr(test, deny(warnings))]
#![feature(async_await)]

use futures::compat::Compat01As03;
use futures::prelude::*;
use std::io;

pub use http;

/// A generic error type.
pub type Fail = Box<dyn std::error::Error + Send + Sync + 'static>;

/// Create an HTTP test.
#[derive(Debug)]
pub struct Client {
    client: hyper::client::Builder,
    method: http::Method,
    uri: http::Uri,
    body: hyper::Body,
}

impl Client {
    /// Create a new instance.
    pub fn new(method: http::Method, uri: http::Uri) -> Self {
        Self {
            client: hyper::client::Client::builder(),
            uri,
            method,
            body: hyper::Body::empty(),
        }
    }

    /// Send a request.
    pub async fn send(self) -> Result<Response, Fail> {
        let req = hyper::Request::builder()
            .method(self.method)
            .uri(self.uri)
            .body(self.body)?;
        let client = hyper::Client::new();
        let res = Compat01As03::new(client.request(req)).await?;
        Ok(Response::new(res))
    }

    /// Send a request and format the response as a `String`.
    pub async fn string(self) -> Result<String, Fail> {
        let mut res = self.send().await?;
        Ok(res.body_string().await?)
    }

    /// Send a request and format the response as a `json`.
    pub async fn json(self) -> Result<(), Fail> {
        unimplemented!();
    }

    /// Send a request and format the response as a `Vec<u8>`.
    pub async fn bytes(self) -> Result<Vec<u8>, Fail> {
        unimplemented!();
    }

    /// Send a request and format the response as a `FormData`.
    pub async fn form_data(self) -> Result<(), Fail> {
        unimplemented!();
    }
}

/// Surf response
#[derive(Debug)]
pub struct Response {
    response: hyper::Response<hyper::Body>
}

impl Response {
    /// Create a new instance
    fn new(response: hyper::Response<hyper::Body>) -> Self {
        Self {
            response,
        }
    }

    /// Remove ownership of the request body, replacing it with an empty body.
    ///
    /// Used primarily for working directly with the body stream.
    pub fn take_body(&mut self) -> hyper::Body {
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
    pub async fn body_bytes(&mut self) -> Result<Vec<u8>, Fail> {
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
    pub async fn body_string(&mut self) -> Result<String, Fail> {
        let body_bytes = self.body_bytes().await?;
        Ok(String::from_utf8(body_bytes).map_err(|_| io::Error::from(io::ErrorKind::InvalidData))?)
    }
}

/// Perform a `GET` request.
pub fn get(uri: impl AsRef<str>) -> Client {
    let uri = uri.as_ref().to_owned().parse().unwrap();
    Client::new(http::Method::GET, uri)
}
