#![feature(async_await)]
type Exception = Box<dyn std::error::Error + Send + Sync + 'static>;

#[runtime::main(runtime_tokio::Tokio)]
async fn main() -> Result<(), Exception> {
    let client = surf::Client::new();

    let req1 = client.get("http://google.com")
        .middleware(surf::middleware::logger::new())
        .recv_string();

    let req2 = client.get("http://google.com")
        .middleware(surf::middleware::logger::new())
        .recv_string();

    futures::future::try_join(req1, req2).await?;
    Ok(())
}
