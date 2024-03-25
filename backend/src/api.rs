use actix_web::{web::{Data, Json},
    post,
    get,
    HttpResponse,
    Result};
use diesel::connection::SimpleConnection;
use diesel::prelude::*;

use crate::database::init_db;

use crate::models::{ User, NewUser };
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

        diesel::insert_into(users).values(&new_usr).execute(&mut init_db());
    }

    HttpResponse::Ok().json("Success".to_string())
}
