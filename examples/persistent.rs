#[async_std::main]
async fn main() -> Result<(), http_types::Error> {
    femme::start(log::LevelFilter::Info)?;

    let client = surf::Client::new();
    let req1 = client.get("https://httpbin.org/get").recv_string();
    let req2 = client.get("https://httpbin.org/get").recv_string();
    futures_util::future::try_join(req1, req2).await?;
    Ok(())
}
