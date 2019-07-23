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

mod http_client;
mod one_off;
mod request;
mod response;

pub mod middleware;

#[doc(inline)]
pub use http;

pub use one_off::{connect, delete, get, head, options, patch, post, put, trace};
pub use request::Request;
pub use response::Response;

/// A generic error type.
pub type Exception = Box<dyn std::error::Error + Send + Sync + 'static>;
