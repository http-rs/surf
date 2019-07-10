#![feature(async_await)]
type Fail = Box<dyn std::error::Error + Send + Sync + 'static>;

#[runtime::main(runtime_tokio::Tokio)]
async fn main() -> Result<(), Fail> {
    let mut res = surf::get("http://google.com").send().await?;
    dbg!(res.into_string().await?);
    Ok(())
}
