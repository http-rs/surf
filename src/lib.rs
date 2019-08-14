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
//! - Swappable HTTP backends
//! - HTTP/2 enabled by default
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
//! # use serde::{Deserialize, Serialize};
//! # #[runtime::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! #[derive(Deserialize, Serialize)]
//! struct Ip {
//!     ip: String
//! }
//!
//! let uri = "https://httpbin.org/post";
//! let data = &Ip { ip: "129.0.0.1".into() };
//! let res = surf::post(uri).body_json(data)?.await?;
//! assert_eq!(res.status(), 200);
//!
//! let uri = "https://api.ipify.org?format=json";
//! let Ip { ip } = surf::get(uri).recv_json().await?;
//! assert!(ip.len() > 10);
//! # Ok(()) }
//! ```
//!
//! And even creating streaming proxies is no trouble at all.
//!
//! ```no_run
//! # #![feature(async_await)]
//! # #[runtime::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! let reader = surf::get("https://img.fyi/q6YvNqP").await?;
//! let res = surf::post("https://box.rs/upload").body(reader).await?;
//! # Ok(()) }
//! ```
//!
//! # Features
//! The following features are available.
//! - __`chttp-client` (default):__ use `chttp` as the HTTP backend.
//! - __`hyper-client`:__ use `hyper` as the HTTP backend.
//! - __`middleware-logger` (default):__ enables logging requests and responses using a middleware.

#![forbid(future_incompatible, rust_2018_idioms)]
#![deny(missing_debug_implementations, nonstandard_style)]
#![warn(missing_docs, unreachable_pub)]
// #![warn(missing_docs, missing_doc_code_examples, unreachable_pub)] TODO(yw): re-enable me
#![cfg_attr(test, deny(warnings))]
#![feature(async_await)]

mod client;
mod http_client;
mod request;
mod response;

pub mod headers;
pub mod middleware;

pub use http;
pub use mime;
pub use url;

pub use client::Client;
pub use request::Request;
pub use response::Response;

#[cfg(feature = "native-client")]
mod one_off;
#[cfg(feature = "native-client")]
pub use one_off::{connect, delete, get, head, options, patch, post, put, trace};

/// A generic error type.
pub type Exception = Box<dyn std::error::Error + Send + Sync + 'static>;
