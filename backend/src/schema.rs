// @generated automatically by Diesel CLI.

diesel::table! {
    tracka_submission_hashes (hash_id) {
        hash_id -> Int4,
        submission_id -> Int4,
        #[max_length = 64]
        hash -> Varchar,
    }
}

diesel::table! {
    tracka_submissions (submission_id) {
        submission_id -> Int4,
        user_id -> Int4,
        powersort_comp -> Int4,
        timsort_comp -> Int4,
        comp_diff -> Float8,
        mcost_diff -> Float8,
        powersort_merge_cost -> Int4,
        timsort_merge_cost -> Int4,
        combined_metric -> Float8,
        submission_size -> Int4,
    }
}

diesel::table! {
    trackb_submissions (submission_id) {
        submission_id -> Int4,
        user_id -> Int4,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Int4,
        #[max_length = 50]
        first_name -> Varchar,
        #[max_length = 50]
        last_name -> Varchar,
        #[max_length = 50]
        email -> Varchar,
    }
}

diesel::joinable!(tracka_submission_hashes -> tracka_submissions (submission_id));
diesel::joinable!(tracka_submissions -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    tracka_submission_hashes,
    tracka_submissions,
    trackb_submissions,
    users,
);
