use super::*;
use actix_web::{http, test, web};
use web::Bytes;

#[actix_rt::test]
async fn test_index_ok() {
    let req = test::TestRequest::with_header("content-type", "text/plain")
        .to_http_request();
    let resp = _index(req).await;
    assert_eq!(resp.status(), http::StatusCode::OK);
}

#[actix_rt::test]
async fn test_hello_ok() {
    let mut app = test::init_service(
        App::new().service(hello)
    ).await;
    let req = test::TestRequest::get().uri("/").to_request();
    let resp = test::call_service(&mut app, req).await;
    let result = test::read_body(resp).await;
    assert_eq!(result, Bytes::from_static(b"Hej med dig!"));
}

#[actix_rt::test]
async fn test_user_ok() {
    let mut app = test::init_service(
        App::new().service(identify)
    ).await;

    let payload = r#"{"username":"Claus","user_id":1967}"#.as_bytes();

    let resp = test::TestRequest::post()
        .uri("/identify")
        .header(header::CONTENT_TYPE, "application/json")
        .set_payload(payload)
        .send_request(&mut app)
        .await;

    let result = test::read_body(resp).await;

    assert_eq!(result, "Welcome Claus, your id is 1967");
}

#[test]
fn my_test() {
    assert_eq!(2, 1 + 1);
}
