use std::convert::TryInto;

use mockito::mock;

use futures_util::future::BoxFuture;
use http_types::Body;

use surf::{middleware::Next, Client, Config, Request, Response};

#[async_std::test]
async fn post_json() -> Result<(), http_types::Error> {
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
    let res = surf::post(mockito::server_url())
        .header("Accept", "application/json")
        .body(Body::from_json(&cat)?)
        .await?;
    m.assert();
    assert_eq!(res.status(), http_types::StatusCode::Ok);
    Ok(())
}

#[async_std::test]
async fn get_json() -> Result<(), http_types::Error> {
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

#[async_std::test]
async fn get_google() -> Result<(), http_types::Error> {
    femme::start(log::LevelFilter::Trace).ok();

    let url = "https://www.google.com";
    let mut res = surf::get(url).await?;
    assert_eq!(res.status(), http_types::StatusCode::Ok);

    let msg = res.body_bytes().await?;
    let msg = String::from_utf8_lossy(&msg);
    println!("recieved: '{}'", msg);
    assert!(msg.contains("<!doctype html>"));
    assert!(msg.contains("<title>Google</title>"));
    assert!(msg.contains("<head>"));
    assert!(msg.contains("</head>"));
    assert!(msg.contains("</script>"));
    assert!(msg.contains("</script>"));

    assert!(msg.contains("<body"));
    assert!(msg.contains("</body>"));
    assert!(msg.contains("</html>"));

    Ok(())
}

#[async_std::test]
async fn get_github() -> Result<(), http_types::Error> {
    femme::start(log::LevelFilter::Trace).ok();

    let url = "https://raw.githubusercontent.com/http-rs/surf/6627d9fc15437aea3c0a69e0b620ae7769ea6765/LICENSE-MIT";
    let mut res = surf::get(url).await?;
    assert_eq!(res.status(), http_types::StatusCode::Ok, "{:?}", &res);

    let msg = res.body_string().await?;

    assert_eq!(
        msg,
        "The MIT License (MIT)

Copyright (c) 2019 Yoshua Wuyts

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the \"Software\"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
"
    );

    Ok(())
}

// TODO(Jeremiah): Re-enable this one httpbin is not broken, or use a mock server.
// #[async_std::test]
// async fn redirect() -> Result<(), http_types::Error> {
//     femme::start(log::LevelFilter::Trace).ok();

//     let url = "https://httpbin.org/redirect/2";
//     let req = surf::get(url);
//     let res = surf::client()
//         .middleware(surf::middleware::Redirect::default())
//         .send(req).await?;
//     assert_eq!(res.status(), http_types::StatusCode::Ok);
//     Ok(())
// }

#[async_std::test]
async fn cloned() -> Result<(), http_types::Error> {
    femme::start(log::LevelFilter::Trace).ok();

    let original_client = surf::client().with(mw_1);
    let cloned_client = original_client.clone().with(mw_2);

    let req = surf::get("https://httpbin.org/get");
    let res = original_client.send(req).await?;
    assert!(res.ext::<Mw1Marker>().is_some());
    assert!(res.ext::<Mw2Marker>().is_none()); // None

    let req = surf::get("https://httpbin.org/get");
    let res = cloned_client.send(req).await?;
    assert!(res.ext::<Mw1Marker>().is_some());
    assert!(res.ext::<Mw2Marker>().is_some()); // Some

    Ok(())
}

struct Mw1Marker;
fn mw_1(
    req: Request,
    client: Client,
    next: Next<'_>,
) -> BoxFuture<Result<Response, http_types::Error>> {
    Box::pin(async move {
        let mut res: Response = next.run(req, client).await?;
        res.insert_ext(Mw1Marker);
        Ok(res)
    })
}
struct Mw2Marker;
fn mw_2(
    req: Request,
    client: Client,
    next: Next<'_>,
) -> BoxFuture<Result<Response, http_types::Error>> {
    Box::pin(async move {
        let mut res = next.run(req, client).await?;
        res.insert_ext(Mw2Marker);
        Ok(res)
    })
}

#[async_std::test]
async fn config_client_headers() -> Result<(), http_types::Error> {
    femme::start(log::LevelFilter::Trace).ok();

    let mut server = tide::new();
    server.at("/").get(|req: tide::Request<()>| async {
        let mut res = tide::Response::new(200);

        for (header_name, header_values) in req {
            res.append_header(header_name, &header_values);
        }

        Ok(res)
    });

    let client: Client = Config::new()
        .set_http_client(server)
        .add_header("X-Header-Name", "X-Header-Values")?
        .try_into()?;

    let res = client.get("http://example.org/").await?;

    assert_eq!(res["X-Header-Name"], "X-Header-Values");

    Ok(())
}
