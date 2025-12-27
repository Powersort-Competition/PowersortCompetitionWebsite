use actix_web::{test, App};
use backend::api::ping;
use actix_web::web::Bytes;

#[actix_rt::test]
async fn test_ping() {
    let mut app = test::init_service(App::new().service(ping)).await;
    let req = test::TestRequest::get().uri("/ping").to_request();
    let resp = test::call_service(&mut app, req).await;

    assert!(resp.status().is_success());
    let body = test::read_body(resp).await;
    assert_eq!(body, Bytes::from_static(b"\"pong\""));
}
