use actix_web::{web::{Data, Json},
    post,
    get,
    HttpResponse,
    Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Response
{
    pub usr_id: Option<String>,
    pub msg: Option<String>
}

#[post("usr_auth")]
pub async fn usr_authenticated(auth_code: Data<Response>) -> HttpResponse
{
    HttpResponse::Ok().json(auth_code)
}