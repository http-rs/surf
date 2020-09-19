use serde_json::value::Value;
use wasm_bindgen_test::*;
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn get() {
    let mut response = surf::get("http://httpbin.org/get")
        .header("custom-header", "header-value")
        .await
        .unwrap();
    let body: Value = response.body_json().await.unwrap();
    assert!(response.status().is_success());
    assert_eq!(body["url"], "http://httpbin.org/get");
    assert_eq!(body["headers"]["Custom-Header"], "header-value");
}

#[wasm_bindgen_test]
async fn post() {
    let mut response = surf::post("http://httpbin.org/post")
        .header("custom-header", "header-value")
        .body(surf::Body::from_string(String::from("body")))
        .await
        .unwrap();
    let body: Value = response.body_json().await.unwrap();
    assert!(response.status().is_success());
    assert_eq!(body["url"], "http://httpbin.org/post");
    assert_eq!(body["data"], "body");
    assert_eq!(body["headers"]["Custom-Header"], "header-value");
}

#[wasm_bindgen_test]
async fn not_found() {
    let response = surf::get("http://httpbin.org/status/404").await.unwrap();
    assert!(response.status().is_client_error());
}

#[wasm_bindgen_test]
async fn server_error() {
    let response = surf::get("http://httpbin.org/status/500").await.unwrap();
    assert!(response.status().is_server_error());
}
