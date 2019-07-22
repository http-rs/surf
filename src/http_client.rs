use crate::{Request, Response};
use futures::future::Boxfuture;

/// An abstract HTTP client.
pub trait HttpClient {
    /// The associated error type.
    type Error;

    /// Perform a request.
    fn request(req: Request) -> BoxFuture<'static, Result<Response, Self::Error>>;
}
