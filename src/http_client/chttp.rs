use super::{Body, HttpClient, Request, Response};

use futures::future::BoxFuture;

use std::sync::Arc;

/// Curl-based HTTP Client.
#[derive(Debug, Default)]
pub struct ChttpClient {
    client: Arc<chttp::HttpClient>,
}

impl ChttpClient {
    /// Create a new instance.
    pub fn new() -> Self {
        Self {
            client: Arc::new(chttp::HttpClient::new()),
        }
    }
}

impl Clone for ChttpClient {
    fn clone(&self) -> Self {
        Self {
            client: self.client.clone(),
        }
    }
}

impl HttpClient for ChttpClient {
    type Error = chttp::Error;

    fn send(&self, req: Request) -> BoxFuture<'static, Result<Response, Self::Error>> {
        let client = self.client.clone();
        Box::pin(async move {
            let (parts, body) = req.into_parts();
            let body = chttp::Body::reader(body);
            let req: http::Request<chttp::Body> = http::Request::from_parts(parts, body);

            let res = client.send_async(req).await?;

            let (parts, body) = res.into_parts();
            let body = Body::from_reader(body);
            let res = http::Response::from_parts(parts, body);
            Ok(res)
        })
    }
}
