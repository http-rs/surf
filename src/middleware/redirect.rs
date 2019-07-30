//! Follow redirects.
//!
//! # Examples
//! ```
//! # #![feature(async_await)]
//! # #[runtime::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! let mut res = surf::get("http://google.com")
//!     .middleware(surf::middleware::follow::new())
//!     .await?;
//! dbg!(res.body_string().await?);
//! # Ok(()) }
//! ```

use futures::future::BoxFuture;

use std::io;

use crate::http_client::HttpClient;
use crate::middleware::{Middleware, Next, Request, Response};

/// Max number of redirects followed.
pub static MAX_REDIRECTS: u8 = 10;

/// Log each request's duration
#[derive(Debug)]
pub struct Redirect;

impl<C: HttpClient> Middleware<C> for Redirect {
    fn handle<'a>(
        &'a self,
        req: Request,
        client: C,
        next: Next<'a, C>,
    ) -> BoxFuture<'a, Result<Response, crate::Exception>> {
        async fn handle<'a, C: HttpClient>(
            _this: &'a Redirect,
            req: Request,
            client: C,
            next: Next<'a, C>,
            redirects_followed: u8,
        ) -> Result<Response, crate::Exception> {
            let method = req.method().clone();

            let res = next.run(req, client).await?;

            if res.status().is_redirection() {
                let location = res.headers().get("Location");
                log::debug!("redirect {:?}", location);
                if redirects_followed >= MAX_REDIRECTS {
                    let err = io::Error::new(io::ErrorKind::Other, "Max redirects reached").into();
                    return Err(err);
                }
            }
            Ok(res)
        }

        Box::pin(handle(self, req, client, next, 0))
    }
}

/// Create a new instance.
pub fn new() -> Redirect {
    Redirect {}
}
