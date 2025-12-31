use actix_web::{test, App};
use backend::api::ping;
use actix_web::web::Bytes;

#[actix_rt::test]
async fn test_ping_regression() {
    let mut app = test::init_service(App::new().service(ping)).await;
    let req = test::TestRequest::get().uri("/ping").to_request();
    let resp = test::call_service(&mut app, req).await;

    assert!(resp.status().is_success());
    let body = test::read_body(resp).await;
    assert_eq!(body, Bytes::from_static(b"\"pong\""));
}

// Note: I would like to add more regression tests for other handlers,
// but they all require a database connection (`init_db` calls internally).
// Without a guaranteed running database in this environment or a way to mock `init_db`
// (which is a standalone function, not easily mocked without refactoring first),
// I cannot write effective integration tests for the other endpoints *before* the refactor.
//
// The refactor itself (introducing dependency injection via Data<Pool>) will actually
// MAKE the code more testable in the future.
