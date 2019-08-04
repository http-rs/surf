use crate::http_client::HttpClient;
use crate::middleware::{Middleware, Next, Request, Response};

use futures::future::BoxFuture;

/// Log each request's duration.
#[derive(Debug)]
pub struct Logger {
    _priv: (),
}

impl Logger {
    /// Create a new instance.
    pub fn new() -> Self{
        Logger {_priv: ()}
    }
}

impl<C: HttpClient> Middleware<C> for Logger {
    #[allow(missing_doc_code_examples)]
    fn handle<'a>(
        &'a self,
        req: Request,
        client: C,
        next: Next<'a, C>,
    ) -> BoxFuture<'a, Result<Response, crate::Exception>> {
        Box::pin(async move {
            log::info!("sending request");

            let res = next.run(req, client).await?;

            let status = res.status();
            let level = if status.is_server_error() {
                log::Level::Error
            } else if status.is_client_error() {
                log::Level::Warn
            } else {
                log::Level::Info
            };

            log::log!(level, "request completed");
            Ok(res)
        })
    }
}
