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

use std::fmt::Debug;

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

    /// Send a request and format the response as a `String`.
    pub async fn send_text(mut self) -> Result<Response<String>, Fail> {
        let req = hyper::Request::builder()
            .method(self.method)
            .uri(self.uri)
            .body(self.body)?;
        let client = hyper::Client::new();
        let res = client.request(req).await?;
        unimplemented!();
    }
}

/// Surf response
#[derive(Debug)]
pub struct Response<Body: Debug> {
    body: Body,
}

impl<Body: Debug> Response<Body> {
    /// Create a new instance
    fn new() -> Self {
        unimplemented!();
    }

    /// Get the body.
    pub fn body(self) -> Body {
        self.body
    }
}

/// Perform a `GET` request.
pub fn get(uri: http::Uri) -> Client {
    Client::new(http::Method::GET, uri)
}
