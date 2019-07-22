//! HTTP Client adapter for Hyper.

use futures::compat::{Compat01As03, Compat as Compat03As01};
use futures::future::BoxFuture;
use futures::io::AsyncRead;
use std::task::{Context, Poll};
use std::pin::Pin;
use std::io;

use super::http_client::{HttpClient, Request, Response};

/// Hyper HTTP Client.
pub struct HyperClient {
    _priv: ()
}

impl HyperClient {
    /// Create a new instance.
    pub fn new() -> Self {
        Self { _priv: () }
    }
}

impl HttpClient for HyperClient {
    type Error = io::Error;

    fn send(req: Request) -> BoxFuture<'static, Result<Response, Self::Error>> {
        Box::pin(async move {
            // Convert the request body.
            let (parts, body) = req.into_parts();
            let byte_stream = Compat03As01::new(ByteStream { reader: body });
            let body = hyper::Body::wrap_stream(byte_stream);
            let req = hyper::Request::from_parts(parts, body);

            // Make a request.
            let client = hyper::Client::new();
            let mut res = Compat01As03::new(client.request(req)).await?;

            // Convert the response body.
            let body = std::mem::replace(res.body_mut(), hyper::Body::empty());
            let body = Compat01As03::new(body)
                .map(|chunk| chunk.map(|chunk| chunk.to_vec()))
                .map_err(|_| io::ErrorKind::InvalidData.into())
                .into_async_read();
            std::mem::replace(res.body_mut(), body);

            Ok(res);
            unimplemented!();
        })
    }
}

struct ByteStream<R: AsyncRead> {
    reader: R
}

impl<R: AsyncRead> futures::Stream for ByteStream<R> {
    type Item = Result<hyper::Chunk, hyper::error::Error>;

    fn poll_next(
        self: Pin<&mut Self>, 
        cx: &mut Context<'_>
    ) -> Poll<Option<Self::Item>> {
        let mut buf = vec![];
        let read = futures::ready!(Pin::new(&mut self.reader).poll_read(cx, &mut buf));
        if read == 0 {
            return None;
        } else {
            buf.shrink_to_fit();
            let chunk = hyper::Chunk::from(buf);
            Some(Ok(chunk))
        }
    }
}
