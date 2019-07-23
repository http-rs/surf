//! Logging middleware.
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

use crate::middleware::{Middleware, Next, Request, Response};
use futures::future::BoxFuture;
use std::time;

/// Log each request's duration
#[derive(Debug)]
pub struct Logger;

impl Middleware for Logger {
    fn handle<'a>(
        &'a self,
        req: Request,
        next: Next<'a>,
    ) -> BoxFuture<'a, Result<Response, crate::Exception>> {
        Box::pin(async move {
            println!("sending request to {}", req.uri());
            let now = time::Instant::now();
            let res = next.run(req).await?;
            println!("request completed ({:?})", now.elapsed());
            Ok(res)
        })
    }
}

/// Create a new instance.
pub fn new() -> Logger {
    Logger {}
}
