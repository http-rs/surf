//! Middleware types

use crate::{Request, Response};
use futures::future::BoxFuture;
use std::sync::Arc;

/// Application context.
#[derive(Debug)]
pub struct Context<State> {
    state: Arc<State>,
    request: Request,
}

/// Middleware that wraps around remaining middleware chain.
pub trait Middleware<State>: 'static + Send + Sync {
    /// Asynchronously handle the request, and return a response.
    fn handle<'a>(&'a self, cx: Context<State>, next: Next<'a, State>) -> BoxFuture<'a, Response>;
}

// This allows functions to work as middleware too.
impl<State, F> Middleware<State> for F
where
    F: Send
        + Sync
        + 'static
        + for<'a> Fn(Context<State>, Next<'a, State>) -> BoxFuture<'a, Response>,
{
    fn handle<'a>(&'a self, cx: Context<State>, next: Next<'a, State>) -> BoxFuture<'a, Response> {
        (self)(cx, next)
    }
}

/// The remainder of a middleware chain, including the endpoint.
#[allow(missing_debug_implementations)]
pub struct Next<'a, State> {
    endpoint:
        &'a (dyn (Fn(Context<State>) -> BoxFuture<'static, Response>) + 'static + Send + Sync),
    next_middleware: &'a [Arc<dyn Middleware<State>>],
}

impl<'a, State: 'static> Next<'a, State> {
    /// Create a new instance
    pub fn new(
        endpoint: &'a (dyn (Fn(Context<State>) -> BoxFuture<'static, Response>)
                 + 'static
                 + Send
                 + Sync),
        next: &'a [Arc<dyn Middleware<State>>],
    ) -> Self {
        Self {
            endpoint,
            next_middleware: next,
        }
    }

    /// Asynchronously execute the remaining middleware chain.
    pub fn run(mut self, cx: Context<State>) -> BoxFuture<'a, Response> {
        if let Some((current, next)) = self.next_middleware.split_first() {
            self.next_middleware = next;
            current.handle(cx, self)
        } else {
            (self.endpoint)(cx)
        }
    }
}
