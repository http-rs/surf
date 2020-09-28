//! HTTP Redirect middleware.
//!
//! # Examples
//!
//! ```no_run
//! # #[async_std::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! let req = surf::get("https://httpbin.org/redirect/2");
//! let client = surf::client().with(surf::middleware::Redirect::new(5));
//! let mut res = client.send(req).await?;
//! dbg!(res.body_string().await?);
//! # Ok(()) }
//! ```

use crate::http::{headers, StatusCode, Url};
use crate::middleware::{Middleware, Next, Request, Response};
use crate::{Client, Result};

// List of acceptible 300-series redirect codes.
const REDIRECT_CODES: &[StatusCode] = &[
    StatusCode::MovedPermanently,
    StatusCode::Found,
    StatusCode::SeeOther,
    StatusCode::TemporaryRedirect,
    StatusCode::PermanentRedirect,
];

/// A middleware which attempts to follow HTTP redirects.
#[derive(Debug)]
pub struct Redirect {
    attempts: u8,
}

impl Redirect {
    /// Create a new instance of the Redirect middleware, which attempts to follow redirects
    /// up to as many times as specified.
    ///
    /// Consider using `Redirect::default()` for the default number of redirect attempts.
    ///
    /// This middleware will follow redirects from the `Location` header if the server returns
    /// any of the following http response codes:
    /// - 301 Moved Permanently
    /// - 302 Found
    /// - 303 See other
    /// - 307 Temporary Redirect
    /// - 308 Permanent Redirect
    ///
    /// # Errors
    ///
    /// An error will be passed through the middleware stack if the value of the `Location`
    /// header is not a validly parsing url.
    ///
    /// # Caveats
    ///
    /// This will presently make at least one additional HTTP request before the actual request to
    /// determine if there is a redirect that should be followed, so as to preserve any request body.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # #[async_std::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// let req = surf::get("https://httpbin.org/redirect/2");
    /// let client = surf::client().with(surf::middleware::Redirect::new(5));
    /// let mut res = client.send(req).await?;
    /// dbg!(res.body_string().await?);
    /// # Ok(()) }
    /// ```
    pub fn new(attempts: u8) -> Self {
        Redirect { attempts }
    }
}

#[async_trait::async_trait]
impl Middleware for Redirect {
    #[allow(missing_doc_code_examples)]
    async fn handle(&self, mut req: Request, client: Client, next: Next<'_>) -> Result<Response> {
        let mut redirect_count: u8 = 0;

        // Note(Jeremiah): This is not ideal.
        //
        // HttpClient is currently too limiting for efficient redirects.
        // We do not want to make unnecessary full requests, but it is
        // presently required due to how Body streams work.
        //
        // Ideally we'd have methods to send a partial request stream,
        // without sending the body, that would potnetially be able to
        // get a server status before we attempt to send the body.
        //
        // As a work around we clone the request first (without the body),
        // and try sending it until we get some status back that is not a
        // redirect.

        while redirect_count < self.attempts {
            redirect_count += 1;
            let r: Request = req.clone();
            let res: Response = client.send(r).await?;
            if REDIRECT_CODES.contains(&res.status()) {
                if let Some(location) = res.header(headers::LOCATION) {
                    *req.as_mut().url_mut() = Url::parse(location.last().as_str())?;
                }
            } else {
                break;
            }
        }

        Ok(next.run(req, client).await?)
    }
}

impl Default for Redirect {
    /// Create a new instance of the Redirect middleware, which attempts to follow up to
    /// 3 redirects (not including the actual request).
    fn default() -> Self {
        Self { attempts: 3 }
    }
}
