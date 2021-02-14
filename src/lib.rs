//! ## surf the web.
//!
//! Surf is a friendly HTTP client built for casual Rustaceans and veterans alike.  It's completely
//! modular, and built directly for `async/await`. Whether it's a quick script, or a cross-platform
//! SDK, Surf will make it work.
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
//! # #[async_std::main]
//! # async fn main() -> surf::Result<()> {
//! let mut res = surf::get("https://httpbin.org/get").await?;
//! dbg!(res.body_string().await?);
//! # Ok(()) }
//! ```
//!
//! It's also possible to skip the intermediate `Response`, and access the response type directly.
//! ```no_run
//! # #[async_std::main]
//! # async fn main() -> surf::Result<()> {
//! dbg!(surf::get("https://httpbin.org/get").recv_string().await?);
//! # Ok(()) }
//! ```
//!
//! Both sending and receiving JSON is real easy too.
//! ```no_run
//! # use serde::{Deserialize, Serialize};
//! # #[async_std::main]
//! # async fn main() -> surf::Result<()> {
//! #[derive(Deserialize, Serialize)]
//! struct Ip {
//!     ip: String
//! }
//!
//! let uri = "https://httpbin.org/post";
//! let data = &Ip { ip: "129.0.0.1".into() };
//! let res = surf::post(uri).body(surf::Body::from_json(data)?).await?;
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
//! # #[async_std::main]
//! # async fn main() -> surf::Result<()> {
//! let req = surf::get("https://img.fyi/q6YvNqP").await?;
//! let body = surf::http::Body::from_reader(req, None);
//! let res = surf::post("https://box.rs/upload").body(body).await?;
//! # Ok(()) }
//! ```
//!
//! # Features
//! The following features are available. The default features are
//! `curl-client`, `middleware-logger`, and `encoding`
//! - __`curl-client` (default):__ use `curl` (through `isahc`) as the HTTP backend.
//! - __`h1-client`:__ use `async-h1` as the HTTP backend.
//! - __`hyper-client`:__ use `hyper` (hyper.rs) as the HTTP backend.
//! - __`wasm-client`:__ use `window.fetch` as the HTTP backend.
//! - __`middleware-logger` (default):__ enables logging requests and responses using a middleware.
//! - __`encoding` (default):__ enables support for body encodings other than utf-8

#![deny(missing_debug_implementations, nonstandard_style)]
#![warn(missing_docs, unreachable_pub, rust_2018_idioms)]
// #![warn(missing_docs, missing_doc_code_examples, unreachable_pub)] TODO(yw): re-enable me
#![cfg_attr(test, deny(warnings))]
#![doc(html_favicon_url = "https://yoshuawuyts.com/assets/http-rs/favicon.ico")]
#![doc(html_logo_url = "https://yoshuawuyts.com/assets/http-rs/logo-rounded.png")]

mod client;
mod request;
mod request_builder;
mod response;

pub mod middleware;
pub mod utils;

pub use http_types::{self as http, Body, Error, Status, StatusCode, Url};

pub use http_client::HttpClient;

pub use client::Client;
pub use request::Request;
pub use request_builder::RequestBuilder;
pub use response::{DecodeError, Response};

cfg_if::cfg_if! {
    if #[cfg(all(
        feature = "default-client",
        any(
            feature = "curl-client",
            all(feature = "wasm-client", target_arch = "wasm32"),
            feature = "h1-client",
            feature = "hyper-client"
        )
    ))] {
        mod one_off;
        pub use one_off::{connect, delete, get, head, options, patch, post, put, trace};

        /// Construct a new `Client`, capable of sending `Request`s and running a middleware stack.
        ///
        /// # Examples
        ///
        /// ```rust
        /// # #[async_std::main]
        /// # async fn main() -> surf::Result<()> {
        /// let client = surf::client();
        ///
        /// let req = surf::get("https://httpbin.org/get");
        /// let res = client.send(req).await?;
        /// # Ok(()) }
        /// ```
        pub fn client() -> Client {
            Client::new()
        }
    }
}

/// A specialized Result type for Surf.
pub type Result<T = Response> = std::result::Result<T, Error>;
