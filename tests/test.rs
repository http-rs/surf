#[runtime::test]
async fn post_json() -> Result<(), surf::Error> {
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

#[runtime::test]
async fn get_json() -> Result<(), surf::Error> {
    #[derive(serde::Deserialize)]
    struct Ip {
        ip: String,
    }

    let uri = "https://api.ipify.org?format=json";
    let ip: Ip = surf::get(uri).recv_json().await?;
    assert!(ip.ip.len() > 10);
    Ok(())
}
