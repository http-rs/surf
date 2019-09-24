#![feature(async_await)]

use accept_encoding::Encoding;
use futures::future::BoxFuture;
use http::{header::CONTENT_ENCODING, StatusCode};
use std::fmt;
use std::io::Read;
use surf::{middleware::HttpClient, Body};

#[derive(Clone, Debug)]
pub struct StubClient(pub Vec<Encoding>);
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

        let mut response_body:Vec<u8> = String::from(r#"
            Lorem ipsum dolor sit amet, consectetur adipiscing elit. Aliquam rutrum et risus sed egestas. Maecenas dapibus enim a posuere
            semper. Cras venenatis et turpis quis aliquam. Suspendisse eget risus in libero tristique consectetur. Ut ut risus cursus, scelerisque
            enim ac, tempus tellus. Vestibulum ac porta felis. Aenean fringilla posuere felis, in blandit enim tristique ut. Sed elementum iaculis
            enim eu commodo.
        "#).into();
        for encoding in &self.0 {
            response_body = match encoding {
                Encoding::Gzip => gzip_compress(&response_body),
                Encoding::Deflate => deflate_compress(&response_body),
                Encoding::Brotli => brotli_compress(&response_body),
                Encoding::Zstd => zstd_compress(&response_body),
                Encoding::Identity => response_body,
            };
        }
        let mut response = Response::new(response_body.into());
        *response.status_mut() = StatusCode::OK;
        // This parses all encodings and appends them to a list separated by ","
        let mut header_value = self
            .0
            .iter()
            .map(|e| String::from(e.to_header_value().to_str().unwrap()))
            .fold(String::new(), |mut line, enc| {
                line.push_str(&enc);
                line.push_str(",");
                line
            });
        // pop the final ","
        header_value.pop();

        response
            .headers_mut()
            .insert(CONTENT_ENCODING, header_value.parse().unwrap());
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
