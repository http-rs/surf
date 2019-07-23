#![feature(async_await)]
type Exception = Box<dyn std::error::Error + Send + Sync + 'static>;

#[runtime::main(runtime_tokio::Tokio)]
async fn main() -> Result<(), Exception> {
    let res = surf::get("http://google.com").await?;
    dbg!(res.into_string().await?);
    Ok(())
}
