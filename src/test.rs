use super::*;
use actix_web::{http, test};

#[actix_rt::test]
async fn test_index_ok() {
    let req = test::TestRequest::with_header("content-type", "text/plain")
        .to_http_request();
    let resp = _index(req).await;
    assert_eq!(resp.status(), http::StatusCode::OK);
}

#[test]
fn my_test() {
    assert_eq!(2, 1 + 1);
}
