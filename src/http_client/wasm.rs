use super::{Body, HttpClient, Request, Response};

use futures::prelude::*;
use wasm_bindgen_futures::futures_0_3::JsFuture;
use futures::future::BoxFuture;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
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
        let url = format!("{}", req.uri());
        let request = web_sys::Request::new_with_str(&url).unwrap();
        let window = window().expect("A global window object could not be found");
        let promise = window.fetch_with_request(&request);
        let _res = JsFuture::from(promise)
            .and_then(|res| {
                debug_assert!(res.is_instance_of::<web_sys::Response>());
                let res: web_sys::Response = res.dyn_into().unwrap();
                futures::future::ok(Response::new(Body::empty()))
            });
        // TODO(yosh): pass this future out as a response.
        Box::pin(futures::future::ok(Response::new(Body::empty())))
    }
}
