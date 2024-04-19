use diesel::prelude::*;
use diesel::sql_types::Integer;
use serde::{ Serialize, Deserialize };

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User
{
    pub user_id: i32,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser
{
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>
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
    pub ratio_comp: f64
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::submissions)]
pub struct NewSubmission
{
    pub user_id: i32,
    pub powersort_comp: i32,
    pub timsort_comp: i32,
    pub ratio_comp: f64
}
