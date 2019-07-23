#![feature(async_await)]
type Exception = Box<dyn std::error::Error + Send + Sync + 'static>;

#[runtime::main(runtime_tokio::Tokio)]
async fn main() -> Result<(), Exception> {
    let string = surf::get("http://google.com")
        .middleware(surf::middleware::logger::new())
        .send()
        .await?;
    dbg!(res.into_string().await?);
    Ok(())
}
