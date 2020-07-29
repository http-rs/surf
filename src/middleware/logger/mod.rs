//! Logging middleware.
//!
//! This middleware is used by default unless the `"middleware-logger"` feature is disabled.
//!
//! # Examples
//!
//! ```no_run
//! # #[async_std::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! let mut res = surf::get("https://httpbin.org/get")
//!     .middleware(surf::middleware::Logger::new())
//!     .await?;
//! dbg!(res.body_string().await?);
//! # Ok(()) }
//! ```

#[cfg(target_arch = "wasm32")]
mod wasm;

#[cfg(target_arch = "wasm32")]
pub use wasm::Logger;

#[cfg(not(target_arch = "wasm32"))]
mod native;

#[cfg(not(target_arch = "wasm32"))]
pub use native::Logger;
