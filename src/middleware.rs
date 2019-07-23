//! Middleware types
//!
//! # Examples
//! ```
//! # #![feature(async_await)]
//! use futures::future::BoxFuture;
//! use surf::middleware::{Next, Middleware};
//! use surf::http_client::{Request, Response};
//! use std::time;
//!
//! /// Log each request's duration
//! #[derive(Debug)]
//! pub struct Logger;
//!
//! impl Middleware for Logger {
//!     fn handle<'a>(
//!         &'a self,
//!         req: Request,
//!         next: Next<'a>,
//!     ) -> BoxFuture<'a, Result<Response, surf::Fail>> {
//!         Box::pin(async move {
//!             println!("sending request to {}", req.uri());
//!             let now = time::Instant::now();
//!             let res = next.run(req).await?;
//!             println!("request completed ({:?})", now.elapsed());
//!             Ok(res)
//!         })
//!     }
//! }
//! ```
//! `Middleware` can also be instantiated using a free function thanks to some convenient trait
//! implementations.
//!
//! ```
//! # #![feature(async_await)]
//! use futures::future::BoxFuture;
//! use surf::middleware::{Next, Middleware};
//! use surf::http_client::{Request, Response};
//! use std::time;
//!
//!
//! fn logger<'a>(req: Request, next: Next<'a>) -> BoxFuture<'a, Result<Response, surf::Fail>> {
//!     Box::pin(async move {
//!         println!("sending request to {}", req.uri());
//!         let now = time::Instant::now();
//!         let res = next.run(req).await?;
//!         println!("request completed ({:?})", now.elapsed());
//!         Ok(res)
//!     })
//! }
//! ```

use crate::Fail;
use crate::http_client::{Request, Response};
use futures::future::BoxFuture;
use std::sync::Arc;

/// Middleware that wraps around remaining middleware chain.
pub trait Middleware: 'static + Send + Sync {
    /// Asynchronously handle the request, and return a response.
    fn handle<'a>(&'a self, req: Request, next: Next<'a>) -> BoxFuture<'a, Result<Response, Fail>>;
}

// This allows functions to work as middleware too.
impl<F> Middleware for F
where
    F: Send
        + Sync
        + 'static
        + for<'a> Fn(Request, Next<'a>) -> BoxFuture<'a, Result<Response, Fail>>,
{
    fn handle<'a>(&'a self, req: Request, next: Next<'a>) -> BoxFuture<'a, Result<Response, Fail>> {
        (self)(req, next)
    }
}

/// The remainder of a middleware chain, including the endpoint.
#[allow(missing_debug_implementations)]
pub struct Next<'a> {
    endpoint:
        &'a (dyn (Fn(Request) -> BoxFuture<'static, Result<Response, Fail>>) + 'static + Send + Sync),
    next_middleware: &'a [Arc<dyn Middleware>],
}

impl<'a> Next<'a> {
    /// Create a new instance
    pub fn new(
        next: &'a [Arc<dyn Middleware>],
        endpoint: &'a (dyn (Fn(Request) -> BoxFuture<'static, Result<Response, Fail>>)
                 + 'static
                 + Send
                 + Sync),
    ) -> Self {
        Self {
            endpoint,
            next_middleware: next,
        }
    }

    /// Asynchronously execute the remaining middleware chain.
    pub fn run(mut self, req: Request) -> BoxFuture<'a, Result<Response, Fail>> {
        if let Some((current, next)) = self.next_middleware.split_first() {
            self.next_middleware = next;
            current.handle(req, self)
        } else {
            (self.endpoint)(req)
        }
    }
}
