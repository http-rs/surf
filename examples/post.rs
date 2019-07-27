#![feature(async_await)]

#[runtime::main]
async fn main() -> Result<(), surf::Exception> {
    femme::start(log::LevelFilter::Info)?;
    let uri = "https://httpbin.org/post";
    let data = serde_json::json!({ "name": "chashu" });
    let res = surf::post(uri).body_json(&data)?.await?;
    assert_eq!(res.status(), 200);
    Ok(())
}
