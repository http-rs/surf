use crate::middleware::{Middleware, Next};
use crate::{Client, Request, Response};

use std::fmt::Arguments;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time;

static COUNTER: AtomicUsize = AtomicUsize::new(0);

/// Log each request's duration.
#[derive(Debug, Default)]
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
        let start_time = time::Instant::now();
        let uri = format!("{}", req.url());
        let method = format!("{}", req.method());
        let id = COUNTER.fetch_add(1, Ordering::Relaxed);
        print(
            log::Level::Info,
            format_args!("sending request"),
            RequestPairs {
                id,
                uri: &uri,
                method: &method,
            },
        );

        let res = next.run(req, client).await?;

        let status = res.status();
        let elapsed = start_time.elapsed();
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
                id,
                elapsed: &format!("{:?}", elapsed),
                status: status.into(),
            },
        );

        Ok(res)
    }
}

struct RequestPairs<'a> {
    id: usize,
    method: &'a str,
    uri: &'a str,
}
impl<'a> log::kv::Source for RequestPairs<'a> {
    fn visit<'kvs>(
        &'kvs self,
        visitor: &mut dyn log::kv::Visitor<'kvs>,
    ) -> Result<(), log::kv::Error> {
        visitor.visit_pair("req.id".into(), self.id.into())?;
        visitor.visit_pair("req.method".into(), self.method.into())?;
        visitor.visit_pair("req.uri".into(), self.uri.into())?;
        Ok(())
    }
}

struct ResponsePairs<'a> {
    id: usize,
    status: u16,
    elapsed: &'a str,
}

impl<'a> log::kv::Source for ResponsePairs<'a> {
    fn visit<'kvs>(
        &'kvs self,
        visitor: &mut dyn log::kv::Visitor<'kvs>,
    ) -> Result<(), log::kv::Error> {
        visitor.visit_pair("req.id".into(), self.id.into())?;
        visitor.visit_pair("req.status".into(), self.status.into())?;
        visitor.visit_pair("elapsed".into(), self.elapsed.into())?;
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
