//! Middleware types
//!
//! # Examples
//! ```no_run
//! use futures::future::BoxFuture;
//! use surf::middleware::{Next, Middleware, Request, Response, HttpClient};
//! use std::time;
//!
//! /// Log each request's duration
//! #[derive(Debug)]
//! pub struct Logger;
//!
//! impl<C: HttpClient> Middleware<C> for Logger {
//!     fn handle<'a>(
//!         &'a self,
//!         req: Request,
//!         client: C,
//!         next: Next<'a, C>,
//!     ) -> BoxFuture<'a, Result<Response, surf::Exception>> {
//!         Box::pin(async move {
//!             println!("sending request to {}", req.uri());
//!             let now = time::Instant::now();
//!             let res = next.run(req, client).await?;
//!             println!("request completed ({:?})", now.elapsed());
//!             Ok(res)
//!         })
//!     }
//! }
//! ```
//! `Middleware` can also be instantiated using a free function thanks to some convenient trait
//! implementations.
//!
//! ```no_run
//! use futures::future::BoxFuture;
//! use surf::middleware::{Next, Middleware, Request, Response, HttpClient};
//! use std::time;
//!
//! fn logger<'a, C: HttpClient>(req: Request, client: C, next: Next<'a, C>) -> BoxFuture<'a, Result<Response, surf::Exception>> {
//!     Box::pin(async move {
//!         println!("sending request to {}", req.uri());
//!         let now = time::Instant::now();
//!         let res = next.run(req, client).await?;
//!         println!("request completed ({:?})", now.elapsed());
//!         Ok(res)
//!     })
//! }
//! ```

#[doc(inline)]
pub use crate::http_client::{Body, HttpClient, Request, Response};

pub mod logger;

use crate::Exception;
use futures::future::BoxFuture;
use std::sync::Arc;

/// Middleware that wraps around remaining middleware chain.
pub trait Middleware<C: HttpClient>: 'static + Send + Sync {
    /// Asynchronously handle the request, and return a response.
    fn handle<'a>(
        &'a self,
        req: Request,
        client: C,
        next: Next<'a, C>,
    ) -> BoxFuture<'a, Result<Response, Exception>>;
}

// This allows functions to work as middleware too.
impl<F, C: HttpClient> Middleware<C> for F
where
    F: Send
        + Sync
        + 'static
        + for<'a> Fn(Request, C, Next<'a, C>) -> BoxFuture<'a, Result<Response, Exception>>,
{
    fn handle<'a>(
        &'a self,
        req: Request,
        client: C,
        next: Next<'a, C>,
    ) -> BoxFuture<'a, Result<Response, Exception>> {
        (self)(req, client, next)
    }
}

/// The remainder of a middleware chain, including the endpoint.
#[allow(missing_debug_implementations)]
pub struct Next<'a, C: HttpClient> {
    next_middleware: &'a [Arc<dyn Middleware<C>>],
    endpoint: &'a (dyn (Fn(Request, C) -> BoxFuture<'static, Result<Response, Exception>>)
             + 'static
             + Send
             + Sync),
}

impl<C: HttpClient> Clone for Next<'_, C> {
    fn clone(&self) -> Self {
        Self {
            next_middleware: self.next_middleware,
            endpoint: self.endpoint,
        }
    }
}

impl<C: HttpClient> Copy for Next<'_, C> {}

impl<'a, C: HttpClient> Next<'a, C> {
    /// Create a new instance
    pub fn new(
        next: &'a [Arc<dyn Middleware<C>>],
        endpoint: &'a (dyn (Fn(Request, C) -> BoxFuture<'static, Result<Response, Exception>>)
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
    pub fn run(mut self, req: Request, client: C) -> BoxFuture<'a, Result<Response, Exception>> {
        if let Some((current, next)) = self.next_middleware.split_first() {
            self.next_middleware = next;
            current.handle(req, client, self)
        } else {
            (self.endpoint)(req, client)
        }
    }
}
