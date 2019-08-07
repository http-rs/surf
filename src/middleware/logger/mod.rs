//! Logging middleware.
//!
//! # Examples
//!
//! ```
//! # #![feature(async_await)]
//! # #[runtime::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! let mut res = surf::get("https://google.com")
//!     .middleware(surf::middleware::logger::new())
//!     .await?;
//! dbg!(res.body_string().await?);
//! # Ok(()) }
//! ```

#[cfg(target_arch = "wasm32")]
mod wasm;

#[cfg(target_arch = "wasm32")]
use wasm::Logger;

#[cfg(not(target_arch = "wasm32"))]
mod native;

#[cfg(not(target_arch = "wasm32"))]
use native::Logger;

/// Create a new instance.
///
/// # Examples
///
/// ```
/// # #![feature(async_await)]
/// # #[runtime::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
/// let mut res = surf::get("https://google.com")
///     .middleware(surf::middleware::logger::new())
///     .await?;
/// dbg!(res.body_string().await?);
/// # Ok(()) }
/// ```
pub fn new() -> Logger {
    Logger::new()
}
