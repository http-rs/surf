use super::{Body, HttpClient, Request, Response};

use futures::future::BoxFuture;
use futures::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::futures_0_3::JsFuture;
use web_sys::window;

use std::pin::Pin;
use std::task::{Context, Poll};
use std::io;

/// WebAssembly HTTP Client.
#[derive(Debug)]
pub struct WasmClient {
    _priv: (),
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
        let fut = Box::pin(async move {
            let url = format!("{}", req.uri());
            let request = web_sys::Request::new_with_str(&url).unwrap();
            let window = window().expect("A global window object could not be found");
            let promise = window.fetch_with_request(&request);
            let res = JsFuture::from(promise).await.unwrap();
            debug_assert!(res.is_instance_of::<web_sys::Response>());
            let res: web_sys::Response = res.dyn_into().unwrap();
            Ok(Response::new(Body::empty()))
        });

        Box::pin(InnerFuture { fut })
    }
}

// This type e
struct InnerFuture {
    fut: Pin<Box<dyn Future<Output = Result<Response, io::Error>> + 'static>>,
}

// This is safe because WASM doesn't have threads yet. Once WASM supports threads we should use a
// thread to park the blocking implementation until it's been completed.
unsafe impl Send for InnerFuture {}

impl Future for InnerFuture {
    type Output = Result<Response, io::Error>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        unsafe { Pin::new_unchecked(&mut self.fut).poll(cx) }
    }
}
