//! Middleware types
//!
//! # Examples
//! ```no_run
//! use surf::middleware::{Next, Middleware};
//! use surf::{Client, Request, Response, Result};
//! use std::time;
//! use std::sync::Arc;
//!
//! /// Log each request's duration
//! #[derive(Debug)]
//! pub struct Logger;
//!
//! #[surf::utils::async_trait]
//! impl Middleware for Logger {
//!     async fn handle(
//!         &self,
//!         req: Request,
//!         client: Client,
//!         next: Next<'_>,
//!     ) -> Result<Response> {
//!         println!("sending request to {}", req.url());
//!         let now = time::Instant::now();
//!         let res = next.run(req, client).await?;
//!         println!("request completed ({:?})", now.elapsed());
//!         Ok(res)
//!     }
//! }
//! ```
//! `Middleware` can also be instantiated using a free function thanks to some convenient trait
//! implementations.
//!
//! ```no_run
//! use futures_util::future::BoxFuture;
//! use surf::middleware::{Next, Middleware};
//! use surf::{Client, Request, Response, Result};
//! use std::time;
//! use std::sync::Arc;
//!
//! fn logger<'a>(req: Request, client: Client, next: Next<'a>) -> BoxFuture<'a, Result<Response>> {
//!     Box::pin(async move {
//!         println!("sending request to {}", req.url());
//!         let now = time::Instant::now();
//!         let res = next.run(req, client).await?;
//!         println!("request completed ({:?})", now.elapsed());
//!         Ok(res)
//!     })
//! }
//! #
//! # #[async_std::main]
//! # async fn main() -> Result<()> {
//! #     surf::client().with(logger);
//! #     Ok(())
//! # }
//! ```
//! When introducing state, some care must be taken to avoid blocking the entire thread.
//!
//! ```no_run
//! use async_std;
//! use surf;
//!
//! // Count how many requests have been made
//! struct RequestCounter {
//!     // You must use the async_std Mutex to avoid blocking the entire thread
//!     seen_count: async_std::sync::Mutex<usize>,
//! }
//!
//! impl Default for RequestCounter {
//!     fn default() -> Self {
//!         Self {
//!             seen_count: async_std::sync::Mutex::new(0),
//!         }
//!     }
//! }
//!
//! #[surf::utils::async_trait]
//! impl surf::middleware::Middleware for RequestCounter {
//!     async fn handle(
//!         &self,
//!         req: surf::Request,
//!         client: surf::Client,
//!         next: surf::middleware::Next<'_>,
//!     ) -> std::result::Result<surf::Response, http_types::Error> {
//!         {
//!             let mut guard = self.seen_count.try_lock().unwrap();
//!             *guard += 1;
//!
//!             println!("Seen count: {}", *guard);
//!         }
//!
//!         next.run(req, client).await
//!     }
//! }
//! ```

use std::sync::Arc;

use crate::{Client, Request, Response, Result};

mod logger;
mod redirect;

pub use logger::Logger;
pub use redirect::Redirect;

use async_trait::async_trait;
use futures_util::future::BoxFuture;

/// Middleware that wraps around remaining middleware chain.
#[async_trait]
pub trait Middleware: 'static + Send + Sync {
    /// Asynchronously handle the request, and return a response.
    async fn handle(&self, req: Request, client: Client, next: Next<'_>) -> Result<Response>;
}

// This allows functions to work as middleware too.
#[async_trait]
impl<F> Middleware for F
where
    F: Send
        + Sync
        + 'static
        + for<'a> Fn(Request, Client, Next<'a>) -> BoxFuture<'a, Result<Response>>,
{
    async fn handle(&self, req: Request, client: Client, next: Next<'_>) -> Result<Response> {
        (self)(req, client, next).await
    }
}

/// The remainder of a middleware chain, including the endpoint.
#[allow(missing_debug_implementations)]
pub struct Next<'a> {
    next_middleware: &'a [Arc<dyn Middleware>],
    endpoint: &'a (dyn (Fn(Request, Client) -> BoxFuture<'static, Result<Response>>)
             + Send
             + Sync
             + 'static),
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
        endpoint: &'a (dyn (Fn(Request, Client) -> BoxFuture<'static, Result<Response>>)
                 + Send
                 + Sync
                 + 'static),
    ) -> Self {
        Self {
            endpoint,
            next_middleware: next,
        }
    }

    /// Asynchronously execute the remaining middleware chain.
    pub fn run(mut self, req: Request, client: Client) -> BoxFuture<'a, Result<Response>> {
        if let Some((current, next)) = self.next_middleware.split_first() {
            self.next_middleware = next;
            current.handle(req, client, self)
        } else {
            (self.endpoint)(req, client)
        }
    }
}
