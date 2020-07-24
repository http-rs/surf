use std::sync::Arc;
use surf::middleware::{HttpClient, Middleware, Next, Request, Response};

struct Printer;

#[surf::utils::async_trait]
impl Middleware for Printer {
    async fn handle(
        &self,
        req: Request,
        client: Arc<dyn HttpClient>,
        next: Next<'_>,
    ) -> Result<Response, http_types::Error> {
        println!("sending a request!");
        let res = next.run(req, client).await?;
        println!("request completed!");
        Ok(res)
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
