//! HTTP Client adapter for Hyper.

use futures::compat::{Compat as Compat03As01, Compat01As03};
use futures::future::BoxFuture;
use futures::prelude::*;
use hyper_tls::HttpsConnector;
use hyper::client::HttpConnector;

use std::io;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::sync::Arc;

use super::{Body, HttpClient, Request, Response};

/// Hyper HTTP Client.
#[derive(Debug)]
pub struct HyperClient {
    client: Arc<hyper::Client<HttpsConnector<HttpConnector>, hyper::Body>>,
}

impl HyperClient {
    /// Create a new instance.
    pub(crate) fn new() -> Self {
        let https = HttpsConnector::new(4).unwrap();
        let client = hyper::Client::builder().build::<_, hyper::Body>(https);
        Self { client: Arc::new(client) }
    }
}

impl Clone for HyperClient {
    fn clone(&self) -> Self {
        Self { client: self.client.clone() }
    }
}

impl HttpClient for HyperClient {
    type Error = hyper::error::Error;

    fn send(&self, req: Request) -> BoxFuture<'static, Result<Response, Self::Error>> {
        let client = self.client.clone();
        Box::pin(async move {
            // Convert the request body.
            let (parts, body) = req.into_parts();
            let byte_stream = Compat03As01::new(ByteStream { reader: body });
            let body = hyper::Body::wrap_stream(byte_stream);
            let req = hyper::Request::from_parts(parts, body);

            // Make a request.
            let res = Compat01As03::new(client.request(req)).await?;

            // Convert the response body.
            let (parts, body) = res.into_parts();
            let body_reader = Compat01As03::new(body)
                .map(|chunk| chunk.map(|chunk| chunk.to_vec()))
                .map_err(|_| io::ErrorKind::InvalidData.into())
                .into_async_read();
            let body = Body::from_reader(Box::new(body_reader));
            let res = http::Response::from_parts(parts, body);

            Ok(res)
        })
    }
}

/// A type that wraps an `AsyncRead` into a `Stream` of `hyper::Chunk`.
struct ByteStream<R: AsyncRead> {
    reader: R,
}

impl<R: AsyncRead + Unpin> futures::Stream for ByteStream<R> {
    type Item = Result<hyper::Chunk, Box<dyn std::error::Error + Send + Sync + 'static>>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        // This is not at all efficient, but that's okay for now.
        let mut buf = vec![];
        let read = futures::ready!(Pin::new(&mut self.reader).poll_read(cx, &mut buf))?;
        if read == 0 {
            return Poll::Ready(None);
        } else {
            buf.shrink_to_fit();
            let chunk = hyper::Chunk::from(buf);
            Poll::Ready(Some(Ok(chunk)))
        }
    }
}
