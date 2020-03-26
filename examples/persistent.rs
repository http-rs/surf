use async_std::task;

// The need for Ok with turbofish is explained here
// https://rust-lang.github.io/async-book/07_workarounds/03_err_in_async_blocks.html
fn main() -> surf::Result<()> {
    femme::start(log::LevelFilter::Info).unwrap();
    task::block_on(async {
        let client = surf::Client::new();
        let req1 = client.get("https://httpbin.org/get").recv_string();
        let req2 = client.get("https://httpbin.org/get").recv_string();
        futures::future::try_join(req1, req2).await?;
        Ok(())
    })
}
