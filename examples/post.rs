use async_std::task;

fn main() {
    femme::start(log::LevelFilter::Info).unwrap();
    task::block_on(async {
        let uri = "https://httpbin.org/post";
        let data = serde_json::json!({ "name": "chashu" });
        let res = surf::post(uri).body_json(&data).unwrap().await.unwrap();
        assert_eq!(res.status(), 200);
    })
}
