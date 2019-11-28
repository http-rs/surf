use async_std::task;

type BoxError = Box<dyn std::error::Error + Send + Sync>;

fn main() -> Result<(), BoxError> {
    femme::start(log::LevelFilter::Info)?;
    task::block_on(async {
        let uri = "https://httpbin.org/post";
        let data = serde_json::json!({ "name": "chashu" });
        // unwrap note: we are definitely passing valid json so we should never panic.
        let res = surf::post(uri).body_json(&data).unwrap().await?;
        assert_eq!(res.status(), 200);
        Ok(())
    })
}
