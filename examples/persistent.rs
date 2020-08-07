#[async_std::main]
async fn main() -> Result<(), http_types::Error> {
    femme::start(log::LevelFilter::Info)?;

    let client = surf::Client::new();
    let res1 = client.recv_string(surf::get("https://httpbin.org/get"));
    let res2 = client.recv_string(surf::get("https://httpbin.org/get"));
    futures_util::future::try_join(res1, res2).await?;
    Ok(())
}
