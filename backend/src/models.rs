use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::MultipartForm;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::tracka_submissions::perc_diff;

#[derive(Debug, MultipartForm)]
pub struct FileDownload {
    #[multipart(rename = "file")]
    pub(crate) file: Vec<TempFile>,
}

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub user_id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

// Track A submission struct.
#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::tracka_submissions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Submission {
    pub submission_id: i32,
    pub user_id: i32,
    pub powersort_comp: i32,
    pub timsort_comp: i32,
    pub perc_diff: f64,
    pub powersort_merge_cost: i32,
    pub timsort_merge_cost: i32,
    pub submission_size: i32,
}

// Track B submission struct.
#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::trackb_submissions)]
pub struct NewSubmission2 {
    pub user_id: i32,
}

// Distinction: we contain the submitter name as part of the submission view.
#[derive(Serialize)]
pub struct SubmissionView {
    pub submission_id: i32,
    pub submitter: String,
    pub user_id: i32,
    pub powersort_comp: i32,
    pub timsort_comp: i32,
    pub perc_diff: f64,
    pub powersort_merge_cost: i32,
    pub timsort_merge_cost: i32,
    pub submission_size: i32,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::tracka_submissions)]
pub struct NewSubmission {
    pub user_id: i32,
    pub powersort_comp: i32,
    pub timsort_comp: i32,
    pub perc_diff: f64,
    pub powersort_merge_cost: i32,
    pub timsort_merge_cost: i32,
    pub submission_size: i32,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::tracka_submission_hashes)]
pub struct SubmissionHash {
    pub submission_id: i32,
    pub hash: String,
}

#[derive(Debug, MultipartForm)]
struct UploadForm {
    #[multipart(rename = "file")]
    file: Vec<TempFile>,
}
