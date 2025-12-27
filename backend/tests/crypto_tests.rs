use backend::crypto::hash_submission;
use std::env;
use serial_test::serial;

#[actix_rt::test]
#[serial]
async fn test_hash_submission() {
    // Set up environment variable
    env::set_var("SUBMISSION_SALT", "test_salt");

    let submission_id = 123;
    let result = hash_submission(submission_id).await;

    // Check length: 32 bytes -> 64 hex characters
    assert_eq!(result.len(), 64);

    // Test consistency
    let result2 = hash_submission(submission_id).await;
    assert_eq!(result, result2);

    // Test different input
    let result3 = hash_submission(124).await;
    assert_ne!(result, result3);
}
