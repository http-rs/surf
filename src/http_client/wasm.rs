use super::{Body, HttpClient, Request, Response};

use futures::future::BoxFuture;
// use wasm_bindgen::prelude::*;
use web_sys::window;

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
            let window = window().expect("A global window object could not be found");
            let res = window.fetch_with_request(&request);
            dbg!(res);
            Ok(Response::new(Body::empty()))
        })
    }
}
