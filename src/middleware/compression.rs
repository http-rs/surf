use crate::{
    http_client::HttpClient,
    middleware::{Body, Middleware, Next, Request, Response},
};
pub use accept_encoding::Encoding;
use async_compression::stream::{BrotliDecoder, DeflateDecoder, GzipDecoder, ZstdDecoder};
use bytes::Bytes;
use futures::{future::BoxFuture, stream::StreamExt, Stream};
use http::{
    header::CONTENT_ENCODING,
    header::{HeaderValue, ACCEPT_ENCODING},
};
use std::io;

static SUPPORTED_ENCODINGS: &str = "gzip, br, deflate, zstd";

pub fn new() -> Compression {
    Compression::new()
}

/// Middleware for automatically handling incoming response compression.
///
/// This middleware currently supports HTTP compression using `gzip`, `deflate`, `br`, and `zstd`.
#[derive(Debug)]
pub struct Compression;

impl Compression {
    /// Creates the Compression middleware.
    pub fn new() -> Self {
        Self {}
    }

    fn parse_encoding(s: &str) -> Result<Encoding, ()> {
        match s {
            "gzip" => Ok(Encoding::Gzip),
            "deflate" => Ok(Encoding::Deflate),
            "br" => Ok(Encoding::Brotli),
            "zstd" => Ok(Encoding::Zstd),
            "identity" => Ok(Encoding::Identity),
            _ => Err(()),
        }
    }

    async fn decoded_stream_to_body<S>(mut decoded_stream: S) -> Body
    where
        S: Stream<Item = io::Result<Bytes>> + Unpin,
    {
        let mut decoded_content: Vec<Vec<u8>> = Vec::new();
        while let Some(bytes) = decoded_stream.next().await {
            decoded_content.push(bytes.unwrap().into_iter().collect::<Vec<u8>>());
        }
        Body::from(decoded_content.into_iter().flatten().collect::<Vec<u8>>())
    }

    async fn decode(&self, req: &mut Response) {
        let encodings = if let Some(hval) = req.headers().get(CONTENT_ENCODING.as_str()) {
            let hval = match hval.to_str() {
                Ok(hval) => hval,
                Err(_) => {
                    return;
                }
            };
            hval.split(',')
                .map(str::trim)
                .rev() // apply decodings in reverse order
                .map(Compression::parse_encoding)
                .collect::<Result<Vec<Encoding>, ()>>()
                .unwrap() //?
        } else {
            return;
        };

        for encoding in encodings {
            match encoding {
                Encoding::Gzip => {
                    let body = std::mem::replace(req.body_mut(), Body::empty());
                    let decoded_stream = GzipDecoder::new(body);
                    *req.body_mut() = Compression::decoded_stream_to_body(decoded_stream).await;
                }
                Encoding::Deflate => {
                    let body = std::mem::replace(req.body_mut(), Body::empty());
                    let decoded_stream = DeflateDecoder::new(body);
                    *req.body_mut() = Compression::decoded_stream_to_body(decoded_stream).await;
                }
                Encoding::Brotli => {
                    let body = std::mem::replace(req.body_mut(), Body::empty());
                    let decoded_stream = BrotliDecoder::new(body);
                    *req.body_mut() = Compression::decoded_stream_to_body(decoded_stream).await;
                }
                Encoding::Zstd => {
                    let body = std::mem::replace(req.body_mut(), Body::empty());
                    let decoded_stream = ZstdDecoder::new(body);
                    *req.body_mut() = Compression::decoded_stream_to_body(decoded_stream).await;
                }
                Encoding::Identity => (),
            }
        }
        // strip the content-encoding header
        req.headers_mut().remove(CONTENT_ENCODING).unwrap();
    }
}

impl<C: HttpClient> Middleware<C> for Compression {
    #[allow(missing_doc_code_examples)]
    fn handle<'a>(
        &'a self,
        mut req: Request,
        client: C,
        next: Next<'a, C>,
    ) -> BoxFuture<'a, Result<Response, crate::Exception>> {
        Box::pin(async move {
            req.headers_mut().insert(
                ACCEPT_ENCODING,
                HeaderValue::from_static(SUPPORTED_ENCODINGS),
            );
            let mut res = next.run(req, client).await?;
            self.decode(&mut res).await;
            Ok(res)
        })
    }
}
