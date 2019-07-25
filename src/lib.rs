//! ## surf the web.
//!
//! Surf is the Rust HTTP client we've always wanted. It's completely modular, and
//! built directly for `async/await`. Whether it's a quick script, or a
//! cross-platform SDK, Surf will make it work.
//!
//! - Multi-platform out of the box
//! - Extensible through a powerful middleware system
//! - Reuses connections through the `Client` interface
//! - Fully streaming requests and responses
//! - TLS/SSL enabled by default
//! - Swappable HTTP backends (`hyper (default)`, `libcurl (wip)`, `fetch (wip)`)
//!
//! # Examples
//! ```no_run
//! # #![feature(async_await)]
//! # #[runtime::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! let mut res = surf::get("https://google.com").await?;
//! dbg!(res.body_string().await?);
//! # Ok(()) }
//! ```
//!
//! It's also possible to skip the intermediate `Response`, and access the response type directly.
//! ```no_run
//! # #![feature(async_await)]
//! # #[runtime::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! dbg!(surf::get("https://google.com").recv_string().await?);
//! # Ok(()) }
//! ```
//!
//! Both sending and receiving JSON is real easy too.
//! ```no_run
//! # #![feature(async_await)]
//! # #[runtime::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! #[derive(serde::Deserialize, serde::Serialize)]
//! struct Ip {
//!     ip: String
//! }
//!
//! let uri = "https://httpbin.org/post";
//! let res = surf::post(uri).json(&Ip { ip: "129.0.0.1".into() })?.await?;
//! assert_eq!(res.status(), 200);
//!
//! let uri = "https://api.ipify.org?format=json";
//! let ip: Ip = surf::get(uri).recv_json().await?;
//! assert!(ip.ip.len() > 10);
//! # Ok(()) }
//! ```

#![forbid(unsafe_code, future_incompatible, rust_2018_idioms)]
#![deny(missing_debug_implementations, nonstandard_style)]
#![warn(missing_docs, missing_doc_code_examples, unreachable_pub)]
#![cfg_attr(test, deny(warnings))]
#![feature(async_await)]

mod client;
mod http_client;
mod one_off;
mod request;
mod response;

pub mod middleware;

#[doc(inline)]
pub use http;

pub use client::Client;
pub use one_off::{connect, delete, get, head, options, patch, post, put, trace};
pub use request::Request;
pub use response::Response;

/// A generic error type.
pub type Exception = Box<dyn std::error::Error + Send + Sync + 'static>;
