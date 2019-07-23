use super::Request;

/// Perform a one-off `GET` request.
pub fn get(uri: impl AsRef<str>) -> Request {
    let uri = uri.as_ref().to_owned().parse().unwrap();
    Request::new(http::Method::GET, uri)
}

/// Perform a one-off `HEAD` request.
pub fn head(uri: impl AsRef<str>) -> Request {
    let uri = uri.as_ref().to_owned().parse().unwrap();
    Request::new(http::Method::HEAD, uri)
}

/// Perform a one-off `POST` request.
pub fn post(uri: impl AsRef<str>) -> Request {
    let uri = uri.as_ref().to_owned().parse().unwrap();
    Request::new(http::Method::POST, uri)
}

/// Perform a one-off `PUT` request.
pub fn put(uri: impl AsRef<str>) -> Request {
    let uri = uri.as_ref().to_owned().parse().unwrap();
    Request::new(http::Method::PUT, uri)
}

/// Perform a one-off `DELETE` request.
pub fn delete(uri: impl AsRef<str>) -> Request {
    let uri = uri.as_ref().to_owned().parse().unwrap();
    Request::new(http::Method::DELETE, uri)
}

/// Perform a one-off `CONNECT` request.
pub fn connect(uri: impl AsRef<str>) -> Request {
    let uri = uri.as_ref().to_owned().parse().unwrap();
    Request::new(http::Method::CONNECT, uri)
}

/// Perform a one-off `OPTIONS` request.
pub fn options(uri: impl AsRef<str>) -> Request {
    let uri = uri.as_ref().to_owned().parse().unwrap();
    Request::new(http::Method::OPTIONS, uri)
}

/// Perform a one-off `TRACE` request.
pub fn trace(uri: impl AsRef<str>) -> Request {
    let uri = uri.as_ref().to_owned().parse().unwrap();
    Request::new(http::Method::TRACE, uri)
}

/// Perform a one-off `PATCH` request.
pub fn patch(uri: impl AsRef<str>) -> Request {
    let uri = uri.as_ref().to_owned().parse().unwrap();
    Request::new(http::Method::PATCH, uri)
}
