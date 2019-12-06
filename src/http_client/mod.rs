//! HTTP Client Interface
use futures::future::BoxFuture;
use futures::io::{AsyncRead, Cursor};

use std::error::Error;
use std::fmt::{self, Debug};
use std::io;
use std::pin::Pin;
use std::task::{Context, Poll};

#[cfg(all(feature = "hyper-client", not(target_arch = "wasm32")))]
pub(crate) mod hyper;

#[cfg(all(feature = "curl-client", not(target_arch = "wasm32")))]
pub(crate) mod isahc;

#[cfg(all(feature = "wasm-client", target_arch = "wasm32"))]
pub(crate) mod wasm;

#[cfg(feature = "native-client")]
pub(crate) mod native;

/// An HTTP Request type with a streaming body.
pub type Request = http::Request<Body>;

/// An HTTP Response type with a streaming body.
pub type Response = http::Response<Body>;

/// __\[unstable\]__ An abstract HTTP client.
///
/// __note that this is only exposed for use in middleware. Building new backing clients is not
/// recommended yet. Once it is we'll likely publish a new `http-client` crate, and re-export this
/// trait from there together with all existing HTTP client implementations.__
///
/// ## Spawning new request from middleware
/// When threading the trait through a layer of middleware, the middleware must be able to perform
/// new requests. In order to enable this we pass an `HttpClient` instance through the middleware,
/// with a `Clone` implementation. In order to spawn a new request, `clone` is called, and a new
/// request is enabled.
///
/// How `Clone` is implemented is up to the implementors, but in an ideal scenario combining this
/// with the `Client` builder will allow for high connection reuse, improving latency.
pub trait HttpClient: Debug + Unpin + Send + Sync + Clone + 'static {
    /// The associated error type.
    type Error: Error + Send + Sync;

    /// Perform a request.
    fn send(&self, req: Request) -> BoxFuture<'static, Result<Response, Self::Error>>;
}

/// The raw body of an http request or response.
///
/// A body is a stream of `Bytes` values, which are shared handles to byte buffers.
/// Both `Body` and `Bytes` values can be easily created from standard owned byte buffer types
/// like `Vec<u8>` or `String`, using the `From` trait.
pub struct Body {
    reader: Box<dyn AsyncRead + Unpin + Send + 'static>,
    /// Intentionally use `u64` over `usize` here.
    /// `usize` won't work if you try to send 10GB file from 32bit host.
    len: Option<u64>,
}

impl Body {
    /// Create a new empty body.
    pub fn empty() -> Self {
        Self {
            reader: Box::new(futures::io::empty()),
            len: Some(0),
        }
    }

    /// Create a new instance from a reader.
    pub fn from_reader(reader: impl AsyncRead + Unpin + Send + 'static) -> Self {
        Self {
            reader: Box::new(reader),
            len: None,
        }
    }
}

impl AsyncRead for Body {
    #[allow(missing_doc_code_examples)]
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        Pin::new(&mut self.reader).poll_read(cx, buf)
    }
}

impl fmt::Debug for Body {
    #[allow(missing_doc_code_examples)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Body").field("reader", &"<hidden>").finish()
    }
}

impl From<Vec<u8>> for Body {
    #[allow(missing_doc_code_examples)]
    #[inline]
    fn from(vec: Vec<u8>) -> Body {
        let len = vec.len() as u64;
        Self {
            reader: Box::new(Cursor::new(vec)),
            len: Some(len),
        }
    }
}

impl<R: AsyncRead + Unpin + Send + 'static> From<Box<R>> for Body {
    /// Converts an `AsyncRead` into a Body.
    #[allow(missing_doc_code_examples)]
    fn from(reader: Box<R>) -> Self {
        Self { reader, len: None }
    }
}
