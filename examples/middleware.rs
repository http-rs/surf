use surf::middleware::{Middleware, Next};
use surf::{Client, Request, Response};

struct Printer;

#[surf::utils::async_trait]
impl Middleware for Printer {
    async fn handle(
        &self,
        req: Request,
        client: Client,
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

    let req = surf::get("https://httpbin.org/get");
    surf::client().with(Printer {}).send(req).await?;
    Ok(())
}
