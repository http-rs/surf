use async_std::task;

// The need for Ok with turbofish is explained here
// https://rust-lang.github.io/async-book/07_workarounds/03_err_in_async_blocks.html
fn main() -> Result<(), surf::Error> {
    femme::start(log::LevelFilter::Info).unwrap();
    task::block_on(async {
        let uri = "https://httpbin.org/post";
        let data = serde_json::json!({ "name": "chashu" });
        let res = surf::post(uri).body_json(&data).unwrap().await?;
        assert_eq!(res.status(), 200);
        Ok::<(), surf::Error>(())
    })
}
