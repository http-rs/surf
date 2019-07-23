use crate::Request;
use crate::http_client::HttpClient;
use crate::http_client::hyper::HyperClient;

use std::fmt::Debug;

/// A persistent HTTP client.
#[derive(Debug)]
pub struct Client<C: HttpClient> {
    http_client: C,
}

impl Client<HyperClient> {
    /// Create a new instance.
    pub fn new() -> Self {
        Self {
            http_client: HyperClient::new(),
        }
    }
}

impl<C: HttpClient + Debug + Unpin> Client<C> {
    /// Submit an HTTP `POST` request.
    pub fn post(&self, uri: impl AsRef<str>, client: C) -> Request<C> {
        let uri = uri.as_ref().to_owned().parse().unwrap();
        Request::with_client(http::Method::POST, uri, client)
    }

}
