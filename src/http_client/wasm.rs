use super::{Body, HttpClient, Request, Response};

use futures::future::BoxFuture;

use std::sync::Arc;

/// WebAssembly HTTP Client.
#[derive(Debug)]
pub struct WasmClient {
    _priv: ()
}

impl WasmClient {
    /// Create a new instance.
    pub fn new() -> Self {
        Self { _priv: () }
    }
}

impl Clone for WasmClient {
    fn clone(&self) -> Self {
        Self { _priv: () }
    }
}

impl HttpClient for WasmClient {
    type Error = chttp::Error;

    fn send(&self, req: Request) -> BoxFuture<'static, Result<Response, Self::Error>> {
        let client = self.client.clone();
        Box::pin(async move {
            let res = client.send_async(req).await?;
            Ok(res)
        })
    }
}
