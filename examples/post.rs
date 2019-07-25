#![feature(async_await)]

#[derive(serde::Deserialize, serde::Serialize)]
struct Cat {
    name: String,
}

#[runtime::main]
async fn main() -> Result<(), surf::Exception> {
    let cat = Cat { name: "Chashu".to_string() };
    let res = surf::post("https://httpbin.org/post")
        .middleware(surf::middleware::logger::new())
        .json(&cat)?
        .await?;
    assert_eq!(res.status(), 200);
    Ok(())
}
