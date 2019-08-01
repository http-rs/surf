use crate::http_client::HttpClient;
use crate::Request;

#[cfg(feature = "chttp-client")]
use crate::http_client::chttp::ChttpClient;

/// A persistent HTTP client.
#[derive(Debug)]
pub struct Client<C: HttpClient> {
    client: C,
}

#[cfg(feature = "chttp-client")]
impl Client<ChttpClient> {
    /// Create a new instance.
    pub fn new() -> Self {
        Self::with_client(ChttpClient::new())
    }
}

impl<C: HttpClient> Client<C> {
    /// Create a new instance with an `http_client::HttpClient` instance.
    pub fn with_client(client: C) -> Self {
        let client = client;
        Self { client }
    }

    /// Perform an HTTP `GET` request using the `Client` connection.
    pub fn get(&self, uri: impl AsRef<str>) -> Request<C> {
        let uri = uri.as_ref().to_owned().parse().unwrap();
        Request::with_client(http::Method::GET, uri, self.client.clone())
    }

    /// Perform an HTTP `HEAD` request using the `Client` connection.
    pub fn head(&self, uri: impl AsRef<str>) -> Request<C> {
        let uri = uri.as_ref().to_owned().parse().unwrap();
        Request::with_client(http::Method::HEAD, uri, self.client.clone())
    }

    /// Perform an HTTP `POST` request using the `Client` connection.
    pub fn post(&self, uri: impl AsRef<str>) -> Request<C> {
        let uri = uri.as_ref().to_owned().parse().unwrap();
        Request::with_client(http::Method::POST, uri, self.client.clone())
    }

    /// Perform an HTTP `PUT` request using the `Client` connection.
    pub fn put(&self, uri: impl AsRef<str>) -> Request<C> {
        let uri = uri.as_ref().to_owned().parse().unwrap();
        Request::with_client(http::Method::PUT, uri, self.client.clone())
    }

    /// Perform an HTTP `DELETE` request using the `Client` connection.
    pub fn delete(&self, uri: impl AsRef<str>) -> Request<C> {
        let uri = uri.as_ref().to_owned().parse().unwrap();
        Request::with_client(http::Method::DELETE, uri, self.client.clone())
    }

    /// Perform an HTTP `CONNECT` request using the `Client` connection.
    pub fn connect(&self, uri: impl AsRef<str>) -> Request<C> {
        let uri = uri.as_ref().to_owned().parse().unwrap();
        Request::with_client(http::Method::CONNECT, uri, self.client.clone())
    }

    /// Perform an HTTP `OPTIONS` request using the `Client` connection.
    pub fn options(&self, uri: impl AsRef<str>) -> Request<C> {
        let uri = uri.as_ref().to_owned().parse().unwrap();
        Request::with_client(http::Method::OPTIONS, uri, self.client.clone())
    }

    /// Perform an HTTP `TRACE` request using the `Client` connection.
    pub fn trace(&self, uri: impl AsRef<str>) -> Request<C> {
        let uri = uri.as_ref().to_owned().parse().unwrap();
        Request::with_client(http::Method::TRACE, uri, self.client.clone())
    }

    /// Perform an HTTP `PATCH` request using the `Client` connection.
    pub fn patch(&self, uri: impl AsRef<str>) -> Request<C> {
        let uri = uri.as_ref().to_owned().parse().unwrap();
        Request::with_client(http::Method::PATCH, uri, self.client.clone())
    }
}
