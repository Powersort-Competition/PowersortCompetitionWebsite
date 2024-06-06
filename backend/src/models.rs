use actix_multipart::form::MultipartForm;
use actix_multipart::form::tempfile::TempFile;
use diesel::prelude::*;
use serde::{ Serialize, Deserialize };

#[derive(Debug, MultipartForm)]
pub struct FileDownload
{
    #[multipart(rename = "file")]
    pub(crate) file: Vec<TempFile>
}

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User
{
    pub user_id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser
{
    pub first_name: String,
    pub last_name: String,
    pub email: String
}

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::submissions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Submission
{
    pub submission_id: i32,
    pub user_id: i32,
    pub powersort_comp: i32,
    pub timsort_comp: i32,
    pub ratio_comp: f64,
    pub powersort_merge_cost: i32,
    pub timsort_merge_cost: i32,
    pub submission_size: i32
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::submissions)]
pub struct NewSubmission
{
    pub user_id: i32,
    pub powersort_comp: i32,
    pub timsort_comp: i32,
    pub ratio_comp: f64,
    pub powersort_merge_cost: i32,
    pub timsort_merge_cost: i32,
    pub submission_size: i32
}

#[derive(Debug, MultipartForm)]
struct UploadForm
{
    #[multipart(rename = "file")]
    file: Vec<TempFile>
}
