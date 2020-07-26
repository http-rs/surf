use futures::future::BoxFuture;
use std::sync::Arc;
use surf::middleware::{HttpClient, Middleware, Next, Request, Response};

struct Printer;

impl Middleware for Printer {
    fn handle<'a>(
        &'a self,
        req: Request,
        client: Arc<dyn HttpClient>,
        next: Next<'a>,
    ) -> BoxFuture<'a, Result<Response, http_types::Error>> {
        Box::pin(async move {
            println!("sending a request!");
            let res = next.run(req, client).await?;
            println!("request completed!");
            Ok(res)
        })
    }
}

#[async_std::main]
async fn main() -> Result<(), http_types::Error> {
    femme::start(log::LevelFilter::Info)?;

    surf::get("https://httpbin.org/get")
        .middleware(Printer {})
        .await?;
    Ok(())
}
