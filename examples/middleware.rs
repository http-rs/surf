use async_std::task;
use futures::future::BoxFuture;
use surf::middleware::{HttpClient, Middleware, Next, Request, Response};

struct Printer;

impl Middleware for Printer {
    fn handle<'a>(
        &'a self,
        req: Request,
        client: Box<dyn HttpClient>,
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

// The need for Ok with turbofish is explained here
// https://rust-lang.github.io/async-book/07_workarounds/03_err_in_async_blocks.html
fn main() -> Result<(), http_types::Error> {
    femme::start(log::LevelFilter::Info)?;

    task::block_on(async {
        surf::get("https://httpbin.org/get")
            .middleware(Printer {})
            .await?;
        Ok::<(), http_types::Error>(())
    })
}
