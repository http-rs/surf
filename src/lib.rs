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

mod request;
mod response;

pub mod middleware;

pub use request::Request;
#[doc(inline)]
pub use http;
pub use response::*;

/// A generic error type.
pub type Fail = Box<dyn std::error::Error + Send + Sync + 'static>;

/// Perform a oneshot `GET` request.
pub fn get(uri: impl AsRef<str>) -> Request {
    let uri = uri.as_ref().to_owned().parse().unwrap();
    Request::new(http::Method::GET, uri)
}

/// Perform a oneshot `POST` request.
pub fn post(uri: impl AsRef<str>) -> Request {
    let uri = uri.as_ref().to_owned().parse().unwrap();
    Request::new(http::Method::POST, uri)
}
