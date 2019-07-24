//! Logging middleware.
//!
//! # Examples
//! ```
//! # #![feature(async_await)]
//! # #[runtime::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! let mut res = surf::get("http://google.com")
//!     .middleware(surf::middleware::logger::new())
//!     .await?;
//! dbg!(res.body_string().await?);
//! # Ok(()) }
//! ```

use crate::http_client::HttpClient;
use crate::middleware::{Middleware, Next, Request, Response};

use futures::future::BoxFuture;
use std::time;

/// Log each request's duration
#[derive(Debug)]
pub struct Logger;

impl<C: HttpClient> Middleware<C> for Logger {
    fn handle<'a>(
        &'a self,
        req: Request,
        client: C,
        next: Next<'a, C>,
    ) -> BoxFuture<'a, Result<Response, crate::Exception>> {
        Box::pin(async move {
            println!("sending request to {}", req.uri());
            let now = time::Instant::now();
            let res = next.run(req, client).await?;
            println!("request completed ({:?})", now.elapsed());
            Ok(res)
        })
    }
}

/// Create a new instance.
pub fn new() -> Logger {
    Logger {}
}
