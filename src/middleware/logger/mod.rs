//! Logging middleware.
//!
//! # Examples
//!
//! ```
//! # use surf::error::BoxError;
//! # #[async_std::main]
//! # async fn main() -> Result<(), BoxError> {
//! let mut res = surf::get("https://httpbin.org/get")
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
/// # use surf::error::BoxError;
/// # #[async_std::main]
/// # async fn main() -> Result<(), BoxError> {
/// let mut res = surf::get("https://httpbin.org/get")
///     .middleware(surf::middleware::logger::new())
///     .await?;
/// dbg!(res.body_string().await?);
/// # Ok(()) }
/// ```
pub fn new() -> Logger {
    Logger::new()
}
