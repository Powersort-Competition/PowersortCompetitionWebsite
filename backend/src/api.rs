use actix_web::{web::{Data, Json},
    post,
    get,
    HttpResponse,
    Result};
use diesel::connection::SimpleConnection;
use diesel::prelude::*;
use diesel::row::NamedRow;
use serde::Serialize;

use crate::database::init_db;

use crate::models::{ User, NewUser, Submission, NewSubmission };
use crate::schema::submissions::dsl::submissions;
use crate::schema::users::dsl::*;

#[get("/ping")]
pub async fn ping() -> HttpResponse
{
    HttpResponse::Ok().json("pong".to_string())
}

/*
Internal function to check if user exists by probing database with email.
 */
fn check_usr_exists(mut conn: PgConnection, email_addr: Option<String>) -> bool
{
    //let res = users.select(User::as_select()).load(&mut conn);
    //println!("{:?}", res.unwrap().get(1).unwrap().email);
    let res = users.filter(email.eq(email_addr)).load::<User>(&mut conn);
    let res_size = res.expect("Error checking if user exists in database!").len();

    if (res_size == 1) { return true }
    else { return false }
}

fn get_user_id(mut conn: PgConnection, email_addr: Option<String>) -> i32
{
    let res = users.filter(email.eq(email_addr)).load::<User>(&mut conn);

    return res.expect("Cannot find user id! This should not happen!").get(0).unwrap().user_id
}

#[post("/logged_in")]
pub async fn login_probe(usr_details: Json<NewUser>) -> HttpResponse
{
    // Check if user already exists in the database via their email address.
    let usr_exists = check_usr_exists(init_db(), usr_details.email.clone());
    
    if (!usr_exists)
    {
        println!("Creating new user in database.");
        // Create a new user in the database.
        let new_usr = NewUser
        {
            first_name: usr_details.first_name.clone(),
            last_name: usr_details.last_name.clone(),
            email: usr_details.email.clone()
        };

        let _ = diesel::insert_into(users).values(&new_usr).execute(&mut init_db());
    }

    HttpResponse::Ok().json("Success".to_string())
}

#[post("/my_user_id")]
pub async fn my_user_id(usr_details: Json<User>) -> HttpResponse
{
    HttpResponse::Ok().json(get_user_id(init_db(), usr_details.email.clone()))
}

#[post("/new_submission")]
pub async fn new_submission(submission: Json<NewSubmission>) -> HttpResponse
{
    println!("Adding new user submission to database.");

    let _new_submission = NewSubmission
    {
        user_id: submission.user_id,
        powersort_comp: submission.powersort_comp.clone(),
        timsort_comp: submission.timsort_comp.clone(),
        ratio_comp: submission.ratio_comp.clone()
    };

    println!("{:?}", _new_submission.ratio_comp);

    let _ = diesel::insert_into(submissions).values(&_new_submission).execute(&mut init_db());

    HttpResponse::Ok().json("Success".to_string())
}
