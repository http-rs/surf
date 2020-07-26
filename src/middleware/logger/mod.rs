//! Logging middleware.
//!
//! This middleware is used by default unless the `"middleware-logger"` feature is disabled.

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
/// ```no_run
/// # #[async_std::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
/// let mut res = surf::get("https://httpbin.org/get")
///     .middleware(surf::middleware::logger::new())
///     .await?;
/// dbg!(res.body_string().await?);
/// # Ok(()) }
/// ```
pub fn new() -> Logger {
    Logger::new()
}
