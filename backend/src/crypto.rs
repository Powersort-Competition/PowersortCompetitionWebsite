use std::env;

use argon2::Argon2;

pub async fn hash_submission(submission_id: i32) -> String {
    println!("Submission ID: {}", submission_id);

    let argon2 = Argon2::default();
    let binding = env::var("SUBMISSION_SALT").expect("SUBMISSION_SALT must be set");
    let salt = binding.as_bytes();
    let mut hashed = [0u8; 32];

    argon2
        .hash_password_into(submission_id.to_string().as_bytes(), salt, &mut hashed)
        .unwrap();

    return hashed
        .to_vec()
        .iter()
        .map(|x| format!("{:02x}", x))
        .collect::<String>();
}
