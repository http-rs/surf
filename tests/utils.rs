#![feature(async_await)]

use accept_encoding::Encoding;
use bytes::Bytes;
use futures::future::BoxFuture;
use http::{
    header::{ACCEPT_ENCODING, CONTENT_ENCODING},
    HeaderValue, StatusCode,
};
use std::fmt;
use std::io::Read;
use surf::{middleware::HttpClient, Body};

#[derive(Clone, Debug)]
pub struct StubClient(pub Encoding);
#[derive(Clone, Debug)]
pub struct StubClientError;

/// An HTTP Request type with a streaming body.
pub type Request = http::Request<Body>;

/// An HTTP Response type with a streaming body.
pub type Response = http::Response<Body>;

impl fmt::Display for StubClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "StubClientError")
    }
}

impl std::error::Error for StubClientError {}

impl HttpClient for StubClient {
    type Error = StubClientError;

    fn send(&self, req: Request) -> BoxFuture<'static, Result<Response, Self::Error>> {
        assert!(req.headers().contains_key(http::header::ACCEPT_ENCODING));

        let response= String::from(r#"
            Lorem ipsum dolor sit amet, consectetur adipiscing elit. Aliquam rutrum et risus sed egestas. Maecenas dapibus enim a posuere
            semper. Cras venenatis et turpis quis aliquam. Suspendisse eget risus in libero tristique consectetur. Ut ut risus cursus, scelerisque
            enim ac, tempus tellus. Vestibulum ac porta felis. Aenean fringilla posuere felis, in blandit enim tristique ut. Sed elementum iaculis
            enim eu commodo.
        "#);
        let mut response = match self.0 {
            Encoding::Gzip => Response::new(Body::from(gzip_compress(response.as_bytes()))),
            Encoding::Deflate => Response::new(Body::from(deflate_compress(response.as_bytes()))),
            Encoding::Brotli => Response::new(Body::from(brotli_compress(response.as_bytes()))),
            Encoding::Zstd => Response::new(Body::from(zstd_compress(response.as_bytes()))),
            Encoding::Identity => Response::new(Body::from(Vec::from(response.as_bytes()))),
        };
        *response.status_mut() = StatusCode::OK;
        response
            .headers_mut()
            .insert(http::header::CONTENT_ENCODING, self.0.to_header_value());
        Box::pin(async move { Ok(response) })
    }
}

fn gzip_compress(bytes: &[u8]) -> Vec<u8> {
    use flate2::{bufread::GzEncoder, Compression};
    read_to_vec(GzEncoder::new(bytes, Compression::fast()))
}

fn deflate_compress(bytes: &[u8]) -> Vec<u8> {
    use flate2::{bufread::DeflateEncoder, Compression};
    read_to_vec(DeflateEncoder::new(bytes, Compression::fast()))
}

fn brotli_compress(bytes: &[u8]) -> Vec<u8> {
    use brotli2::bufread::BrotliEncoder;
    read_to_vec(BrotliEncoder::new(bytes, 1))
}

fn zstd_compress(bytes: &[u8]) -> Vec<u8> {
    use libzstd::stream::read::Encoder;
    use libzstd::DEFAULT_COMPRESSION_LEVEL;
    read_to_vec(Encoder::new(bytes, DEFAULT_COMPRESSION_LEVEL).unwrap())
}

fn read_to_vec(mut read: impl Read) -> Vec<u8> {
    let mut output = vec![];
    read.read_to_end(&mut output).unwrap();
    output
}
