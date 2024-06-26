CREATE TABLE tracka_submission_hashes (
    hash_id SERIAL PRIMARY KEY,
    submission_id INT NOT NULL,
    hash VARCHAR(64) NOT NULL,
    CONSTRAINT fk_submission_id
                         FOREIGN KEY(submission_id)
                         REFERENCES tracka_submissions(submission_id)
                         ON DELETE CASCADE
);