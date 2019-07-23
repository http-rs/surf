//! HTTP client framework.
//!
//! # Examples
//! ```
//! # #![feature(async_await)]
//! # #[runtime::main(runtime_tokio::Tokio)]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! let res = surf::get("http://google.com")
//!     .middleware(surf::middleware::logger::new())
//!     .send().await?;
//! dbg!(res.into_string().await?);
//! # Ok(()) }
//! ```

#![forbid(unsafe_code, future_incompatible, rust_2018_idioms)]
#![deny(missing_debug_implementations, nonstandard_style)]
#![warn(missing_docs, missing_doc_code_examples, unreachable_pub)]
#![cfg_attr(test, deny(warnings))]
#![feature(async_await)]

mod request;
mod response;
mod http_client;

pub mod middleware;

#[doc(inline)]
pub use http;

pub use response::Response;
pub use request::Request;

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
