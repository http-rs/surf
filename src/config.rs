//! Configuration for `HttpClient`s.

use std::sync::Arc;
use std::{collections::HashMap, fmt::Debug, time::Duration};

use http_client::{Config as HttpConfig, HttpClient};
use http_types::headers::{HeaderName, HeaderValues, ToHeaderValues};

use crate::http::Url;
use crate::Result;

/// Configuration for `surf::Client`s and their underlying HTTP clients.
///
/// ```
/// use std::convert::TryInto;
/// use surf::{Client, Config, Url};
///
/// # #[async_std::main]
/// # async fn main() -> surf::Result<()> {
/// let client: Client = Config::new()
///     .set_base_url(Url::parse("https://example.org")?)
///     .try_into()?;
///
/// let mut response = client.get("/").await?;
///
/// println!("{}", response.body_string().await?);
/// # Ok(())
/// # }
/// ```
#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct Config {
    /// The base URL for a client. All request URLs will be relative to this URL.
    ///
    /// Note: a trailing slash is significant.
    /// Without it, the last path component is considered to be a “file” name
    /// to be removed to get at the “directory” that is used as the base.
    pub base_url: Option<Url>,
    /// Headers to be applied to every request made by this client.
    pub headers: HashMap<HeaderName, HeaderValues>,
    /// Underlying HTTP client config.
    pub http_config: HttpConfig,
    /// Optional custom http client.
    pub http_client: Option<Arc<dyn HttpClient>>,
}

impl Config {
    /// Construct new empty config.
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Config {
    fn default() -> Self {
        HttpConfig::default().into()
    }
}

impl Config {
    /// Adds a header to be added to every request by this client.
    ///
    /// Default: No extra headers.
    ///
    /// ```
    /// use std::convert::TryInto;
    /// use surf::{Client, Config};
    /// use surf::http::auth::BasicAuth;
    ///
    /// # fn main() -> surf::Result<()> {
    /// let auth = BasicAuth::new("Username", "Password");
    ///
    /// let client: Client = Config::new()
    ///     .add_header(auth.name(), auth.value())?
    ///     .try_into()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn add_header(
        mut self,
        name: impl Into<HeaderName>,
        values: impl ToHeaderValues,
    ) -> Result<Self> {
        self.headers
            .insert(name.into(), values.to_header_values()?.collect());
        Ok(self)
    }

    /// Sets the base URL for this client. All request URLs will be relative to this URL.
    ///
    /// Note: a trailing slash is significant.
    /// Without it, the last path component is considered to be a “file” name
    /// to be removed to get at the “directory” that is used as the base.
    ///
    /// Default: `None` (internally).
    ///
    /// ```
    /// use std::convert::TryInto;
    /// use surf::{Client, Config, Url};
    ///
    /// # fn main() -> surf::Result<()> {
    /// let client: Client = Config::new()
    ///     .set_base_url(Url::parse("https://example.org")?)
    ///     .try_into()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_base_url(mut self, base: Url) -> Self {
        self.base_url = Some(base);
        self
    }

    /// Set HTTP/1.1 `keep-alive` (connection pooling).
    ///
    /// Default: `true`.
    ///
    /// Note: Does nothing on `wasm-client` (or `native-client` on `wasm32`).
    pub fn set_http_keep_alive(mut self, keep_alive: bool) -> Self {
        self.http_config.http_keep_alive = keep_alive;
        self
    }

    /// Set TCP `NO_DELAY`.
    ///
    /// Default: `false`.
    ///
    /// Note: Does nothing on `wasm-client` (or `native-client` on `wasm32`).
    pub fn set_tcp_no_delay(mut self, no_delay: bool) -> Self {
        self.http_config.tcp_no_delay = no_delay;
        self
    }

    /// Set connection timeout duration.
    ///
    /// Passing `None` will remove the timeout.
    ///
    /// Default: `Some(Duration::from_secs(60))`.
    ///
    /// ```
    /// use std::convert::TryInto;
    /// use std::time::Duration;
    /// use surf::{Client, Config};
    ///
    /// # fn main() -> surf::Result<()> {
    /// let client: Client = Config::new()
    ///     .set_timeout(Some(Duration::from_secs(5)))
    ///     .try_into()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_timeout(mut self, timeout: Option<Duration>) -> Self {
        self.http_config.timeout = timeout;
        self
    }

    /// Set the maximum number of simultaneous connections that this client is allowed to keep open to individual hosts at one time.
    ///
    /// Default: `50`.
    /// This number is based on a few random benchmarks and see whatever gave decent perf vs resource use in Orogene.
    ///
    /// Note: The behavior of this is different depending on the backend in use.
    /// - `h1-client`: `0` is disallowed and asserts as otherwise it would cause a semaphore deadlock.
    /// - `curl-client`: `0` allows for limitless connections per host.
    /// - `hyper-client`: No effect. Hyper does not support such an option.
    /// - `wasm-client`: No effect. Web browsers do not support such an option.
    pub fn set_max_connections_per_host(mut self, max_connections_per_host: usize) -> Self {
        self.http_config.max_connections_per_host = max_connections_per_host;
        self
    }

    /// Override the http client entirely.
    ///
    /// When using this, any underlying `http_client::Config` http configuration will be ignored.
    ///
    /// ```
    /// use std::convert::TryInto;
    /// use surf::{Client, Config};
    ///
    /// # fn main() -> surf::Result<()> {
    /// // Connect directly to a Tide server, e.g. for testing.
    /// let server = tide::new();
    ///
    /// let client: Client = Config::new()
    ///     .set_http_client(server)
    ///     .try_into()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_http_client(mut self, http_client: impl HttpClient) -> Self {
        self.http_client = Some(Arc::new(http_client));
        self
    }

    /// Set TLS Configuration (Rustls)
    #[cfg_attr(feature = "docs", doc(cfg(feature = "h1-client-rustls")))]
    #[cfg(feature = "h1-client-rustls")]
    pub fn set_tls_config(
        mut self,
        tls_config: Option<std::sync::Arc<rustls_crate::ClientConfig>>,
    ) -> Self {
        self.http_config.tls_config = tls_config;
        self
    }
    /// Set TLS Configuration (Native TLS)
    #[cfg_attr(feature = "docs", doc(cfg(feature = "h1-client")))]
    #[cfg(feature = "h1-client")]
    pub fn set_tls_config(
        mut self,
        tls_config: Option<std::sync::Arc<async_native_tls::TlsConnector>>,
    ) -> Self {
        self.http_config.tls_config = tls_config;
        self
    }
}

impl AsRef<HttpConfig> for Config {
    fn as_ref(&self) -> &HttpConfig {
        &self.http_config
    }
}

impl From<HttpConfig> for Config {
    fn from(http_config: HttpConfig) -> Self {
        Self {
            base_url: None,
            headers: HashMap::new(),
            http_config,
            http_client: None,
        }
    }
}
