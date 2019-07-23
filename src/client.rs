use crate::Request;
use crate::http_client::HttpClient;
use crate::http_client::hyper::HyperClient;
use std::sync::Arc;

/// A persistent HTTP client.
#[derive(Debug)]
pub struct Client<C: HttpClient> {
    client: C,
}

impl Client<HyperClient> {
    /// Create a new instance.
    pub fn new() -> Self {
        Self::with_client(HyperClient::new())
    }
}

impl<C: HttpClient> Client<C> {
    /// Create a new instance with an `http_client::HttpClient` instance.
    pub fn with_client(client: C) -> Self {
        let client = client;
        Self { client }
    }

    /// Submit an HTTP `POST` request.
    pub fn post(&self, uri: impl AsRef<str>) -> Request<C> {
        let uri = uri.as_ref().to_owned().parse().unwrap();
        Request::with_client(http::Method::POST, uri, self.client)
    }
}
