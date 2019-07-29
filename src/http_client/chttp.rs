use futures::future::BoxFuture;
use chttp;

use super::{Body, HttpClient, Request, Response};

/// Curl HTTP Client.
///
/// ## Performance
/// Libcurl is not thread safe, which means unfortunatley we cannot reuse connections or multiplex.
#[derive(Debug)]
pub struct ChttpClient {
    _priv: ()
}

impl ChttpClient {
    /// Create a new instance.
    pub fn new() -> Self {
        Self { _priv: () }
    }
}

impl Clone for ChttpClient {
    fn clone(&self) -> Self {
        Self::new()
    }
}

impl HttpClient for ChttpClient {
    type Error = chttp::Error;

    fn send(&self, req: Request) -> BoxFuture<'static, Result<Response, Self::Error>> {
        Box::pin(async move {
            unimplemented!();
        })
    }
}

