use actix_web::{web::{Data, Json},
    post,
    get,
    HttpResponse,
    Result};
use serde::{Deserialize, Serialize};

// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct Response
// {
//     pub usr_id: String,
//     pub msg: String
// }
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User
{
    pub email: String,
    pub first_name: String,
    pub last_name: String
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserSubmission
{
    pub email: String,
    pub powersort_comps: Vec<i32>,
    pub timsort_comps: Vec<i32>
}

#[get("/ping")]
pub async fn ping() -> HttpResponse
{
    HttpResponse::Ok().json("pong".to_string())
}

#[get("/get_user_comps/{email}")]
pub async fn get_user_comps() -> HttpResponse
{
    let data = UserSubmission
    {
        email: "test".to_string(),
        powersort_comps: vec![1, 3, 4],
        timsort_comps: vec![3, 4, 6]
    };
    
    HttpResponse::Ok().json(data)
}