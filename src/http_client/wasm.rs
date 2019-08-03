use super::{HttpClient, Request, Response};

use futures::future::BoxFuture;
use wasm_bindgen::prelude::*;
use web_sys::window;

// use std::sync::Arc;

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
    type Error = std::io::Error;

    fn send(&self, req: Request) -> BoxFuture<'static, Result<Response, Self::Error>> {
        Box::pin(async move {
            let url = format!("{}", req.uri());
            let request = web_sys::Request::new_with_str(&url).unwrap();
            let res = window().unwrap().fetch_with_request(&request);
            dbg!(res);
            unimplemented!();
        })
    }
}
