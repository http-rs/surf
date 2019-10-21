use async_std::task;

fn main() {
    femme::start(log::LevelFilter::Info).unwrap();
    task::block_on(async {
        let client = surf::Client::new();
        let req1 = client.get("https://httpbin.org/get").recv_string();
        let req2 = client.get("https://httpbin.org/get").recv_string();
        futures::future::try_join(req1, req2).await.unwrap();
    })
}
