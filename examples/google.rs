#![feature(async_await)]
type Exception = Box<dyn std::error::Error + Send + Sync + 'static>;

#[runtime::main(runtime_tokio::Tokio)]
// #[runtime::main]
async fn main() -> Result<(), Exception> {
    let string = surf::get("https://google.com")
        .middleware(surf::middleware::logger::new())
        .recv_string()
        .await?;
    dbg!(string);
    Ok(())
}
