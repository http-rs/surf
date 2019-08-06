use super::{Body, HttpClient, Request, Response};

use futures::future::BoxFuture;
use futures::prelude::*;
use js_sys::{ArrayBuffer, Uint8Array};
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
            let window = window().expect("A global window object could not be found");
            let url = format!("{}", req.uri());

            // Do a request
            let request = web_sys::Request::new_with_str(&url).unwrap();
            let promise = window.fetch_with_request(&request);
            let resp = JsFuture::from(promise).await.unwrap();
            debug_assert!(resp.is_instance_of::<web_sys::Response>());
            let res: web_sys::Response = resp.dyn_into().unwrap();

            // Get the body
            let promise = res.array_buffer().unwrap();
            let resp = JsFuture::from(promise).await.unwrap();
            debug_assert!(resp.is_instance_of::<js_sys::ArrayBuffer>());
            let buf: ArrayBuffer = resp.dyn_into().unwrap();
            let slice = Uint8Array::new(&buf);
            let mut dest: Vec<u8> = vec![0; slice.length() as usize];
            slice.copy_to(&mut dest);

            // Create our response
            let _headers = res.headers();
            let mut response = Response::new(Body::from(dest));
            *response.status_mut() = http::StatusCode::from_u16(res.status()).unwrap();

            Ok(response)
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
        // This is safe because we're only using this future as a pass-through for the inner
        // future, in order to implement `Send`. If it's safe to poll the inner future, it's safe
        // to proxy it too.
        unsafe { Pin::new_unchecked(&mut self.fut).poll(cx) }
    }
}
