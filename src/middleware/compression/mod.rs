//! Compression middleware.
//!
//! Middleware for automatically handling incoming response compression. It will automatically
//! set your ACCEPTED_ENCODING header and decodes the incomming response from the server (if necessary).
//! This middleware currently supports HTTP compression using `gzip`, `deflate`, `br`, and `zstd`.
//! # Examples
//!
//! ```
//! # #![feature(async_await)]
//! # #[runtime::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! let mut res = surf::get("https://httpbin.org/brotli")
//!     .middleware(surf::middleware::compression::new())
//!     .await?;
//! dbg!(res.body_json().await?);
//! # Ok(()) }
//! ```
mod compression;
mod compression_error;
use compression::Compression;

/// Adds the compression middleware to the request.
///
/// # Examples
///
/// ```
/// # #![feature(async_await)]
/// # #[runtime::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
/// let mut res = surf::get("https://httpbin.org/gzip")
///     .middleware(surf::middleware::compression::new())
///     .await?;
/// dbg!(res.body_json().await?);
/// # Ok(()) }
/// ```
pub fn new() -> Compression {
    Compression::new()
}
