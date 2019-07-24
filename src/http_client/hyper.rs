//! HTTP Client adapter for Hyper.

use futures::compat::{Compat as Compat03As01, Compat01As03};
use futures::future::BoxFuture;
use futures::ready;
use futures::prelude::*;
use hyper::client::connect as hyper_connect;
use hyper_tls::HttpsConnector;
use runtime::net::TcpStream;
use native_tls::TlsConnector;

use std::io;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

use super::{Body, HttpClient, Request, Response};

/// Hyper HTTP Client.
#[derive(Debug)]
pub struct HyperClient {
    client: Arc<hyper::Client<HttpsConnector<RuntimeTcpConnector>, hyper::Body>>,
}

impl HyperClient {
    /// Create a new instance.
    pub(crate) fn new() -> Self {
        // Create a TLS decoder, TCP stream, and combine them into a `Connector` to be passed to
        // Hyper.
        let tcp_connector = RuntimeTcpConnector::new();
        let tls_connector = TlsConnector::new().unwrap();
        let https = HttpsConnector::from((tcp_connector, tls_connector));

        // Create the Hyper client with the `Connector`, and make sure we use `runtime` to spawn
        // futures.
        let client = hyper::Client::builder()
            .executor(Compat03As01::new(runtime::task::Spawner::new()))
            .build::<_, hyper::Body>(https);
        Self {
            client: Arc::new(client),
        }
    }
}

impl Clone for HyperClient {
    fn clone(&self) -> Self {
        Self {
            client: self.client.clone(),
        }
    }
}

impl HttpClient for HyperClient {
    type Error = hyper::error::Error;

    fn send(&self, req: Request) -> BoxFuture<'static, Result<Response, Self::Error>> {
        Box::pin(async move {
            // Convert the request body.
            let (parts, body) = req.into_parts();
            let byte_stream = Compat03As01::new(ChunkStream { reader: body });
            let body = hyper::Body::wrap_stream(byte_stream);
            let req = hyper::Request::from_parts(parts, body);

            // Make a request.
            let client = hyper::Client::new();
            let res = Compat01As03::new(client.request(req)).await?;

            // Convert the response body.
            let (parts, body) = res.into_parts();
            let body_stream = Compat01As03::new(body)
                .map(|c| dbg!(c))
                .map(|chunk| chunk.map(|chunk| chunk.to_vec()))
                .map_err(|_| io::ErrorKind::InvalidData.into());
            let body_reader = IntoAsyncRead::new(body_stream);
            let body = Body::from_reader(Box::new(body_reader));
            let res = http::Response::from_parts(parts, body);

            Ok(res)
        })
    }
}

/// An `AsyncRead` for the [`into_async_read`](super::TryStreamExt::into_async_read) combinator.
#[derive(Debug)]
#[must_use = "streams do nothing unless polled"]
struct IntoAsyncRead<St>
where
    St: TryStream<Error = io::Error> + Unpin,
    St::Ok: AsRef<[u8]>,
{
    stream: St,
    state: ReadState<St::Ok>,
}

impl<St> Unpin for IntoAsyncRead<St>
where
    St: TryStream<Error = io::Error> + Unpin,
    St::Ok: AsRef<[u8]>,
{
}

#[derive(Debug)]
enum ReadState<T: AsRef<[u8]>> {
    Ready { chunk: T, chunk_start: usize },
    PendingChunk,
    Eof,
}

impl<St> IntoAsyncRead<St>
where
    St: TryStream<Error = io::Error> + Unpin,
    St::Ok: AsRef<[u8]>,
{
    pub(super) fn new(stream: St) -> Self {
        IntoAsyncRead {
            stream,
            state: ReadState::PendingChunk,
        }
    }
}

impl<St> AsyncRead for IntoAsyncRead<St>
where
    St: TryStream<Error = io::Error> + Unpin,
    St::Ok: AsRef<[u8]>,
{
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        loop {
            match &mut self.state {
                ReadState::Ready { chunk, chunk_start } => {
                    let chunk = chunk.as_ref();
                    let len = std::cmp::min(buf.len(), chunk.len() - *chunk_start);

                    buf[..len].copy_from_slice(
                        &chunk[*chunk_start..*chunk_start + len],
                    );
                    *chunk_start += len;

                    if chunk.len() == *chunk_start {
                        self.state = ReadState::PendingChunk;
                    }

                    return Poll::Ready(Ok(len));
                }
                ReadState::PendingChunk => {
                    match ready!(self.stream.try_poll_next_unpin(cx)) {
                        Some(Ok(chunk)) => {
                            if !chunk.as_ref().is_empty() {
                                self.state = ReadState::Ready {
                                    chunk,
                                    chunk_start: 0,
                                };
                            }
                        }
                        Some(Err(err)) => {
                            self.state = ReadState::Eof;
                            return Poll::Ready(Err(err));
                        }
                        None => {
                            self.state = ReadState::Eof;
                            return Poll::Ready(Ok(0));
                        }
                    }
                }
                ReadState::Eof => {
                    return Poll::Ready(Ok(0));
                }
            }
        }
    }
}

/// A type that wraps an `AsyncRead` into a `Stream` of `hyper::Chunk`. Used for writing data to a
/// Hyper response.
struct ChunkStream<R: AsyncRead> {
    reader: R,
}

impl<R: AsyncRead + Unpin> futures::Stream for ChunkStream<R> {
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

/// The struct passed to Hyper so we can use arbitrary `AsyncRead` + `AsyncWrite` streams to make
/// connections.
pub(crate) struct RuntimeTcpConnector {
    _priv: (),
}

impl RuntimeTcpConnector {
    /// Create a new instance
    pub(crate) fn new() -> Self {
        Self { _priv: () }
    }
}

impl hyper_connect::Connect for RuntimeTcpConnector {
    type Transport = Compat03As01<TcpStream>;
    type Error = io::Error;
    type Future = Compat03As01<
        Pin<
            Box<
                dyn Future<
                        Output = Result<(Self::Transport, hyper_connect::Connected), Self::Error>,
                    > + Send,
            >,
        >,
    >;

    fn connect(&self, dest: hyper_connect::Destination) -> Self::Future {
        Compat03As01::new(Box::pin(async move {
            let port = match dest.port() {
                Some(port) => port,
                None if dest.scheme() == "https" => 443,
                None => 80
            };

            // Create a TcpStream and return it.
            let tcp_stream = TcpStream::connect((dest.host(), port)).await?;
            Ok((Compat03As01::new(tcp_stream), hyper_connect::Connected::new()))
        }))
    }
}
