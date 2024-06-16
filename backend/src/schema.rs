// @generated automatically by Diesel CLI.

diesel::table! {
    submissions (submission_id) {
        submission_id -> Int4,
        user_id -> Int4,
        powersort_comp -> Int4,
        timsort_comp -> Int4,
        perc_diff -> Float8,
        powersort_merge_cost -> Int4,
        timsort_merge_cost -> Int4,
        submission_size -> Int4,
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

diesel::joinable!(submissions -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    submissions,
    users,
);
