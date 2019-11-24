use async_std::task;
use futures::future::BoxFuture;
use surf::middleware::{HttpClient, Middleware, Next, Request, Response};

struct Printer;

impl<C: HttpClient> Middleware<C> for Printer {
    fn handle<'a>(
        &'a self,
        req: Request,
        client: C,
        next: Next<'a, C>,
    ) -> BoxFuture<'a, Result<Response, surf::Error>> {
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
fn main() -> Result<(), surf::Error> {
    femme::start(log::LevelFilter::Info)?;

    task::block_on(async {
        surf::get("https://httpbin.org/get")
            .middleware(Printer {})
            .await?;
        Ok::<(), surf::Error>(())
    })
}
