//! HTTP Client Interface
use futures::future::BoxFuture;
use futures::io::AsyncRead;

use std::pin::Pin;
use std::task::{Context, Poll};
use std::{fmt, io};

/// An HTTP Request type with a streaming body.
pub type Request = http::Request<Body>;

/// An HTTP Response type with a streaming body.
pub type Response = http::Response<Body>;

/// An abstract HTTP client.
pub trait HttpClient {
    /// The associated error type.
    type Error;

    /// Perform a request.
    fn send(req: Request) -> BoxFuture<'static, Result<Response, Self::Error>>;
}

/// The raw body of an http request or response.
///
/// A body is a stream of `Bytes` values, which are shared handles to byte buffers.
/// Both `Body` and `Bytes` values can be easily created from standard owned byte buffer types
/// like `Vec<u8>` or `String`, using the `From` trait.
pub struct Body {
    reader: Box<dyn AsyncRead + Unpin + Send + 'static>,
}

impl AsyncRead for Body {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        Pin::new(&mut self.reader).poll_read(cx, buf)
    }
}

impl fmt::Debug for Body {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Body").field("reader", &"<hidden>").finish()
    }
}
