#![cfg(feature = "native-client")]

use mockito::mock;

#[async_std::test]
async fn post_json() -> Result<(), surf::Exception> {
    #[derive(serde::Deserialize, serde::Serialize)]
    struct Cat {
        name: String,
    }

    let cat = Cat {
        name: "Chashu".to_string(),
    };

    let m = mock("POST", "/")
        .with_status(200)
        .match_body(&serde_json::to_string(&cat)?[..])
        .with_body(&serde_json::to_string(&cat)?[..])
        .create();
    let res = surf::post(mockito::server_url()).body_json(&cat)?.await?;
    m.assert();
    assert_eq!(res.status(), 200);
    Ok(())
}

#[async_std::test]
async fn get_json() -> Result<(), surf::Exception> {
    #[derive(serde::Deserialize)]
    struct Message {
        message: String,
    }
    let m = mock("GET", "/")
        .with_status(200)
        .with_body(r#"{"message": "hello, world!"}"#)
        .create();
    let uri = &mockito::server_url();
    let msg: Message = surf::get(uri).recv_json().await?;
    m.assert();
    assert_eq!(msg.message, "hello, world!");
    Ok(())
}
