//! Middleware types
//!
//! # Examples
//! ```no_run
//! use futures::future::BoxFuture;
//! use surf::middleware::{Next, Middleware, Request, Response, HttpClient};
//! use std::error::Error;
//! use std::time;
//! use std::sync::Arc;
//!
//! /// Log each request's duration
//! #[derive(Debug)]
//! pub struct Logger;
//!
//! impl Middleware for Logger {
//!     fn handle<'a>(
//!         &'a self,
//!         req: Request,
//!         client: Arc<dyn HttpClient>,
//!         next: Next<'a>,
//!     ) -> BoxFuture<'a, Result<Response, http_types::Error>> {
//!         Box::pin(async move {
//!             println!("sending request to {}", req.url());
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
//! use std::sync::Arc;
//!
//! fn logger<'a>(req: Request, client: Arc<dyn HttpClient>, next: Next<'a>) -> BoxFuture<'a, Result<Response, http_types::Error>> {
//!     Box::pin(async move {
//!         println!("sending request to {}", req.url());
//!         let now = time::Instant::now();
//!         let res = next.run(req, client).await?;
//!         println!("request completed ({:?})", now.elapsed());
//!         Ok(res)
//!     })
//! }
//! ```

use std::sync::Arc;

#[doc(inline)]
pub use http_client::{Body, HttpClient, Request, Response};

pub mod logger;
mod redirect;

pub use redirect::Redirect;

use futures::future::BoxFuture;
use http_types::Error;

/// Middleware that wraps around remaining middleware chain.
pub trait Middleware: 'static + Send + Sync {
    /// Asynchronously handle the request, and return a response.
    fn handle<'a>(
        &'a self,
        req: Request,
        client: Arc<dyn HttpClient>,
        next: Next<'a>,
    ) -> BoxFuture<'a, Result<Response, Error>>;
}

// This allows functions to work as middleware too.
impl<F> Middleware for F
where
    F: Send
        + Sync
        + 'static
        + for<'a> Fn(Request, Arc<dyn HttpClient>, Next<'a>) -> BoxFuture<'a, Result<Response, Error>>,
{
    fn handle<'a>(
        &'a self,
        req: Request,
        client: Arc<dyn HttpClient>,
        next: Next<'a>,
    ) -> BoxFuture<'a, Result<Response, Error>> {
        (self)(req, client, next)
    }
}

/// The remainder of a middleware chain, including the endpoint.
#[allow(missing_debug_implementations)]
pub struct Next<'a> {
    next_middleware: &'a [Arc<dyn Middleware>],
    endpoint: &'a (dyn (Fn(Request, Arc<dyn HttpClient>) -> BoxFuture<'static, Result<Response, Error>>)
             + 'static
             + Send
             + Sync),
}

impl Clone for Next<'_> {
    fn clone(&self) -> Self {
        Self {
            next_middleware: self.next_middleware,
            endpoint: self.endpoint,
        }
    }
}

impl Copy for Next<'_> {}

impl<'a> Next<'a> {
    /// Create a new instance
    pub fn new(
        next: &'a [Arc<dyn Middleware>],
        endpoint: &'a (dyn (Fn(Request, Arc<dyn HttpClient>) -> BoxFuture<'static, Result<Response, Error>>)
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
    pub fn run(
        mut self,
        req: Request,
        client: Arc<dyn HttpClient>,
    ) -> BoxFuture<'a, Result<Response, Error>> {
        if let Some((current, next)) = self.next_middleware.split_first() {
            self.next_middleware = next;
            current.handle(req, client, self)
        } else {
            (self.endpoint)(req, client)
        }
    }
}
