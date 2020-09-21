//! Logging middleware.
//!
//! This middleware is used by default unless the `"middleware-logger"` feature is disabled.
//!
//! # Examples
//!
//! ```no_run
//! # #[async_std::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! let req = surf::get("https://httpbin.org/get");
//! let mut res = surf::client()
//!     .with(surf::middleware::Logger::new())
//!     .send(req).await?;
//! dbg!(res.body_string().await?);
//! # Ok(()) }
//! ```

cfg_if::cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        mod wasm;
        pub use wasm::Logger;
    } else {
        mod native;
        pub use native::Logger;
    }
}
