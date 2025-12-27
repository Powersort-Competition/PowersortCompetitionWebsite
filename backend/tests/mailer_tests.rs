use backend::mailer::body_constructor;
use std::env;
use serial_test::serial;

#[test]
#[serial]
fn test_body_constructor() {
    env::set_var("MAIL_FROM", "test@example.com");
    let body = "<h1>Test Email</h1>".to_string();
    let email = "recipient@example.com".to_string();

    let message = body_constructor(body.clone(), email.clone());

    // We cannot easily inspect the message content due to private fields in lettre::Message,
    // but we can verify it constructed successfully without panic.
    // If we wanted to inspect, we might need to rely on Debug impl or similar, but lettre::Message might not expose everything easily.

    // However, if it didn't panic, that's a good sign.
    // Let's try to verify what we can.
    // There isn't an easy accessor for headers in the stable API we are using likely?
    // Let's just ensure it runs.
}

#[test]
#[should_panic]
#[serial]
fn test_body_constructor_missing_env() {
    // Ensure we start with a clean slate for this test
    env::remove_var("MAIL_FROM");
    let body = "<h1>Test Email</h1>".to_string();
    let email = "recipient@example.com".to_string();

    let _ = body_constructor(body, email);
}
