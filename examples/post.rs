#![feature(async_await)]

#[runtime::main]
async fn main() -> Result<(), surf::Exception> {
    #[derive(serde::Serialize)]
    struct Cat {
        name: String,
    }

    let uri = "https://httpbin.org/post";
    let res = surf::post(uri).json(&Cat { name: "Chashu".into() })?.await?;
    assert_eq!(res.status(), 200);
    Ok(())
}
