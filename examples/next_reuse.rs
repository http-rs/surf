use async_std::task;
use futures::future::BoxFuture;
use futures::io::AsyncReadExt;
use surf::middleware::{Body, HttpClient, Middleware, Next, Request, Response};

struct Doubler;

impl<C: HttpClient> Middleware<C> for Doubler {
    fn handle<'a>(
        &'a self,
        req: Request,
        client: C,
        next: Next<'a, C>,
    ) -> BoxFuture<'a, Result<Response, surf::Exception>> {
        if req.method().is_safe() {
            let mut new_req = Request::new(Body::empty());
            *new_req.method_mut() = req.method().clone();
            *new_req.uri_mut() = req.uri().clone();
            *new_req.version_mut() = req.version().clone();
            *new_req.headers_mut() = req.headers().clone();
            Box::pin(async move {
                let mut buf = Vec::new();
                let (res1, res2) =
                    futures::future::join(next.run(req, client.clone()), next.run(new_req, client))
                        .await;

                let res = res1?;
                let mut body = res.into_body();
                body.read_to_end(&mut buf).await?;

                let mut res = res2?;
                let mut body = std::mem::replace(res.body_mut(), Body::empty());
                body.read_to_end(&mut buf).await?;

                *res.body_mut() = Body::from(buf);
                Ok(res)
            })
        } else {
            next.run(req, client)
        }
    }
}

fn main() {
    femme::start(log::LevelFilter::Info).unwrap();
    task::block_on(async {
        let mut res = surf::get("https://httpbin.org/get")
            .middleware(Doubler {})
            .await.unwrap();
        dbg!(&res);
        let body = res.body_bytes().await.unwrap();
        let body = String::from_utf8_lossy(&body);
        println!("{}", body);
    })
}
