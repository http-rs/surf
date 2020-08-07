#[async_std::main]
async fn main() -> Result<(), http_types::Error> {
    femme::start(log::LevelFilter::Info)?;

    let uri = "https://httpbin.org/post";
    let data = serde_json::json!({ "name": "chashu" });
    let res = surf::post(uri)
        .body(http_types::Body::from_json(&data)?)
        .await?;
    assert_eq!(res.status(), http_types::StatusCode::Ok);
    Ok(())
}
