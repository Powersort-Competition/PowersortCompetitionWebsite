CREATE TABLE tracka_submissions (
    submission_id SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    powersort_comp integer NOT NULL,
    timsort_comp integer NOT NULL,
    perc_diff float NOT NULL,
    powersort_merge_cost integer NOT NULL,
    timsort_merge_cost integer NOT NULL,
    submission_size integer NOT NULL,
    CONSTRAINT fk_user_id
                         FOREIGN KEY(user_id)
                         REFERENCES users(user_id)
                         ON DELETE CASCADE
)