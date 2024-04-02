CREATE TABLE submissions (
    submission_id SERIAL PRIMARY KEY,
    user_id INT,
    powersort_comp integer NOT NULL,
    timsort_comp integer NOT NULL,
    ratio_comp float NOT NULL,
    CONSTRAINT fk_user_id
                         FOREIGN KEY(user_id)
                         REFERENCES users(user_id)
                         ON DELETE CASCADE
)