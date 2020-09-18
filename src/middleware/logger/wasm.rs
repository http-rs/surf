use crate::middleware::{Client, Middleware, Next, Request, Response};
use std::fmt::Arguments;

/// Log each request's duration.
#[derive(Debug)]
pub struct Logger {
    _priv: (),
}

impl Logger {
    /// Create a new instance.
    pub fn new() -> Self {
        Logger { _priv: () }
    }
}

#[async_trait::async_trait]
impl Middleware for Logger {
    #[allow(missing_doc_code_examples)]
    async fn handle(
        &self,
        req: Request,
        client: Client,
        next: Next<'_>,
    ) -> Result<Response, http_types::Error> {
        let uri = format!("{}", req.url());
        let method = format!("{}", req.method());
        print(
            log::Level::Info,
            format_args!("sending request"),
            RequestPairs {
                uri: &uri,
                method: &method,
            },
        );

        let res = next.run(req, client).await?;

        let status = res.status();
        let level = if status.is_server_error() {
            log::Level::Error
        } else if status.is_client_error() {
            log::Level::Warn
        } else {
            log::Level::Info
        };

        print(
            level,
            format_args!("request completed"),
            ResponsePairs {
                status: status.into(),
            },
        );
        Ok(res)
    }
}

struct RequestPairs<'a> {
    method: &'a str,
    uri: &'a str,
}
impl<'a> log::kv::Source for RequestPairs<'a> {
    fn visit<'kvs>(
        &'kvs self,
        visitor: &mut dyn log::kv::Visitor<'kvs>,
    ) -> Result<(), log::kv::Error> {
        visitor.visit_pair("req.method".into(), self.method.into())?;
        visitor.visit_pair("req.uri".into(), self.uri.into())?;
        Ok(())
    }
}

struct ResponsePairs {
    status: u16,
}

impl log::kv::Source for ResponsePairs {
    fn visit<'kvs>(
        &'kvs self,
        visitor: &mut dyn log::kv::Visitor<'kvs>,
    ) -> Result<(), log::kv::Error> {
        visitor.visit_pair("req.status".into(), self.status.into())?;
        Ok(())
    }
}

fn print(level: log::Level, msg: Arguments<'_>, key_values: impl log::kv::Source) {
    if level <= log::STATIC_MAX_LEVEL && level <= log::max_level() {
        log::logger().log(
            &log::Record::builder()
                .args(msg)
                .key_values(&key_values)
                .level(level)
                .target(module_path!())
                .module_path(Some(module_path!()))
                .file(Some(file!()))
                .line(Some(line!()))
                .build(),
        );
    }
}
