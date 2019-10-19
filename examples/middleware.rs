use futures::future::BoxFuture;
use surf::middleware::{HttpClient, Middleware, Next, Request, Response};

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

use async_std::task;

fn main() {
    femme::start(log::LevelFilter::Info);

    task::block_on(async {
        surf::get("https://httpbin.org/get")
            .middleware(Printer {})
            .await.unwrap();
    });
    ()
}
