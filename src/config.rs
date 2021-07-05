//! Configuration for `HttpClient`s.

use std::sync::Arc;
use std::{collections::HashMap, fmt::Debug, time::Duration};

use http_client::{Config as ClientConfig, HttpClient};
use http_types::headers::{HeaderName, HeaderValues, ToHeaderValues};

use crate::http::Url;
use crate::Result;

/// Configuration for `surf::Client`s and their underlying HTTP clients.
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
    pub client_config: ClientConfig,
    /// Optional custom http client.
    pub http_client: Option<Arc<dyn HttpClient>>,
}

impl Config {
    /// Construct new empty config.
    pub fn new() -> Self {
        ClientConfig::default().into()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Config {
    /// Sets the base URL for this client. All request URLs will be relative to this URL.
    ///
    /// Note: a trailing slash is significant.
    /// Without it, the last path component is considered to be a “file” name
    /// to be removed to get at the “directory” that is used as the base.
    pub fn set_base_url(mut self, base: Url) -> Self {
        self.base_url = Some(base);
        self
    }

    /// Adds a header to be added to every request by this client.
    pub fn add_header(
        mut self,
        name: impl Into<HeaderName>,
        values: impl ToHeaderValues,
    ) -> Result<Self> {
        self.headers
            .insert(name.into(), values.to_header_values()?.collect());
        Ok(self)
    }

    /// Set HTTP/1.1 `keep-alive` (connection pooling).
    pub fn set_http_keep_alive(mut self, keep_alive: bool) -> Self {
        self.client_config.http_keep_alive = keep_alive;
        self
    }

    /// Set TCP `NO_DELAY`.
    pub fn set_tcp_no_delay(mut self, no_delay: bool) -> Self {
        self.client_config.tcp_no_delay = no_delay;
        self
    }

    /// Set connection timeout duration.
    pub fn set_timeout(mut self, timeout: Option<Duration>) -> Self {
        self.client_config.timeout = timeout;
        self
    }

    /// Override the http client entirely.
    pub fn set_http_client(mut self, http_client: impl HttpClient) -> Self {
        self.http_client = Some(Arc::new(http_client));
        self
    }
}

impl AsRef<ClientConfig> for Config {
    fn as_ref(&self) -> &ClientConfig {
        &self.client_config
    }
}

impl From<ClientConfig> for Config {
    fn from(client_config: ClientConfig) -> Self {
        Self {
            base_url: None,
            headers: HashMap::new(),
            client_config,
            http_client: None,
        }
    }
}
