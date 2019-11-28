use surf::error::BoxError;

#[async_std::test]
async fn post_json() -> Result<(), BoxError> {
    #[derive(serde::Deserialize, serde::Serialize)]
    struct Cat {
        name: String,
    }

    let cat = Cat {
        name: "Chashu".to_string(),
    };

    let res = surf::post("https://httpbin.org/post")
        .body_json(&cat)?
        .await?;
    assert_eq!(res.status(), 200);
    Ok(())
}

#[async_std::test]
async fn get_json() -> Result<(), BoxError> {
    #[derive(serde::Deserialize)]
    struct Ip {
        ip: String,
    }

    let uri = "https://api.ipify.org?format=json";
    let ip: Ip = surf::get(uri).recv_json().await?;
    assert!(ip.ip.len() > 10);
    Ok(())
}
