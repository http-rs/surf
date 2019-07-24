//! HTTP Client adapter for Hyper.

use futures::compat::{Compat as Compat03As01, Compat01As03};
use futures::future::BoxFuture;
use futures::prelude::*;
use hyper::client::connect as hyper_connect;
use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;
use runtime::net::TcpStream;

use std::io;
use std::net::{SocketAddr, ToSocketAddrs};
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

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
pub struct HyperConnector;

impl hyper_connect::Connect for HyperConnector {
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
            let addr = (
                dest.host(),
                dest.port().unwrap_or_else(|| match dest.scheme() {
                    "https" => 443,
                    _ => 80,
                }),
            );

            let addr = addr.to_socket_addrs()?.next();
            let addr = addr.ok_or_else(|| {
                io::Error::new(io::ErrorKind::Other, "destination resolved to no address")
            })?;

            // Create the TcpStream and send it to the TcpConnector
            let tcp_stream = TcpStream::connect(addr).await?;
            Ok::<TcpStream, io::Error>(tcp_stream);
            unimplemented!()
        }))
    }
}

// /// The `Future` returned for each call to `connect` inside `Hyper`.
// pub struct HyperTcpConnector(Result<TcpStream, Option<io::Error>>);

// impl HyperTcpConnector {
//     pub fn new(dest: hyper_connect::Destination) -> Self {
//         Self(Ok(stream)).compat();
//         unimplemented!();
//     }
// }

// impl Future for HyperTcpConnector {
//     type Output = Result<(Compat03As01<TcpStream>, hyper_connect::Connected), io::Error>;

//     fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
//         let connector = self.0.as_mut().map_err(|x| x.take().unwrap())?;
//         let stream = futures::ready!(connector.poll_unpin(cx)?);
//         Poll::Ready(Ok((stream.compat(), hyper_connect::Connected::new())))
//     }
// }

// pub struct TcpConnector {
//     addr: SocketAddr,
//     stream: Option<TcpStream>,
// }

// impl Future for TcpConnector {
//     type Output = io::Result<TcpStream>;

//     fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
//         let stream = self.stream.as_mut().unwrap();
//         futures::ready!(stream.async_connect(&self.addr, cx))?;
//         let stream = self.stream.take().unwrap();
//         Poll::Ready(Ok(stream))
//     }
// }
