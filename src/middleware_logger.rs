//! Logging middleware.

use futures::future::BoxFuture;
use crate::middleware::{Next, Middleware};
use crate::http_client::{Request, Response};
use std::time;

/// Log each request's duration
#[derive(Debug)]
pub struct Logger;

impl Middleware for Logger {
    fn handle<'a>(
        &'a self,
        req: Request,
        next: Next<'a>,
    ) -> BoxFuture<'a, Result<Response, crate::Fail>> {
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
