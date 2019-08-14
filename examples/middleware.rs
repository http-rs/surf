#![feature(async_await)]

use surf::middleware::{Middleware, Request, Response, Next, HttpClient};
use futures::future::BoxFuture;

struct Printer;

impl<C: HttpClient> Middleware<C> for Printer {
    fn handle<'a>(
        &'a self,
        req: Request,
        client: C,
        next: Next<'a, C>,
    ) -> BoxFuture<'a, Result<Response, surf::Exception>> {
        Box::pin(async move {
            println!("sending a request!");
            let res = next.run(req, client).await?;
            println!("request completed!");
            Ok(res)
        })
    }
}

#[runtime::main]
async fn main() -> Result<(), surf::Exception> {
    femme::start(log::LevelFilter::Info)?;
    surf::get("https://httpbin.org/get")
        .middleware(Printer {})
        .await?;
    Ok(())
}
