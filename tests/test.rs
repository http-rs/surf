#![feature(async_await)]

mod utils;
use utils::StubClient;
use accept_encoding::Encoding;
use surf::middleware::compression::Compression;


#[runtime::test]
async fn post_json() -> Result<(), surf::Exception> {
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
async fn get_json() -> Result<(), surf::Exception> {
    #[derive(serde::Deserialize)]
    struct Ip {
        ip: String,
    }

    let uri = "https://api.ipify.org?format=json";
    let ip: Ip = surf::get(uri).recv_json().await?;
    assert!(ip.ip.len() > 10);
    Ok(())
}

#[runtime::test]
async fn decode_response() -> Result<(), surf::Exception> {
    let content= String::from(r#"
            Lorem ipsum dolor sit amet, consectetur adipiscing elit. Aliquam rutrum et risus sed egestas. Maecenas dapibus enim a posuere
            semper. Cras venenatis et turpis quis aliquam. Suspendisse eget risus in libero tristique consectetur. Ut ut risus cursus, scelerisque
            enim ac, tempus tellus. Vestibulum ac porta felis. Aenean fringilla posuere felis, in blandit enim tristique ut. Sed elementum iaculis
            enim eu commodo.
        "#);
    let encodings = vec![Encoding::Gzip, Encoding::Brotli, Encoding::Deflate, Encoding::Identity, Encoding::Zstd];
    for encoding in encodings {
        let client = surf::Client::with_client(StubClient(encoding));
        let uncompressed = client.get("http://tmp.net")
            .middleware(Compression::new())
            .recv_string()
            .await?;
        assert_eq!(content, uncompressed);
    }
    Ok(())
}