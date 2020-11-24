use futures_util::io::AsyncReadExt;
use surf::http;
use surf::middleware::{Middleware, Next};
use surf::{Body, Client, Request, Response};

struct Doubler;

#[surf::utils::async_trait]
impl Middleware for Doubler {
    async fn handle(
        &self,
        req: Request,
        client: Client,
        next: Next<'_>,
    ) -> Result<Response, http_types::Error> {
        if req.method().is_safe() {
            let mut new_req = http_types::Request::new(req.method(), req.url().clone());
            let http_req: &http::Request = req.as_ref();
            new_req.set_version(http_req.version());
            let mut new_req: Request = new_req.into();

            for (name, value) in &req {
                new_req.insert_header(name, value);
            }

            let mut buf = Vec::new();
            let (res1, res2) = futures_util::future::join(
                next.run(req, client.clone()),
                next.run(new_req, client),
            )
            .await;

            let mut res = res1?;
            res.read_to_end(&mut buf).await?;

            let mut res = res2?;
            res.read_to_end(&mut buf).await?;
            res.set_body(Body::from(buf));
            Ok(res)
        } else {
            next.run(req, client).await
        }
    }
}

#[async_std::main]
async fn main() -> Result<(), http_types::Error> {
    femme::start(log::LevelFilter::Info)?;

    let req = surf::get("https://httpbin.org/get");
    let mut res = surf::client().with(Doubler {}).send(req).await?;
    dbg!(&res);
    let body = res.body_bytes().await?;
    let body = String::from_utf8_lossy(&body);
    println!("{}", body);
    Ok(())
}
