use std::env;
use std::fs;
use std::io::Read;

use actix_multipart::form::MultipartForm;
use actix_web::{get, HttpRequest, HttpResponse, post, web::Json};
use actix_web::web::Path;
use diesel::prelude::*;

use crate::database::init_db;
use crate::mailer;
use crate::models::{FileDownload, NewSubmission, NewUser, Submission, User};
use crate::schema::submissions::{perc_diff, submission_id, submission_size};
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
#[allow(unused_parens)]
fn check_usr_exists(mut conn: PgConnection, email_addr: String) -> bool
{
    //let res = users.select(User::as_select()).load(&mut conn);
    //println!("{:?}", res.unwrap().get(1).unwrap().email);
    let res = users.filter(email.eq(email_addr)).load::<User>(&mut conn);
    let res_size = res.expect("Error checking if user exists in database!").len();

    if (res_size == 1) { return true }
    else { return false }
}

fn dispatch_mail_receipt(usr_details: Json<User>, submission: NewSubmission)
{
    let body = format!("Hello! Your submission has been recorded successfully. \
                        \n\n<br> <br> Powersort Comparison: {} \
                        \n<br> Timsort Comparison: {} \
                        \n<br> Difference in merge costs: {} \
                        \n<br> Powersort Merge Cost: {} \
                        \n<br> Timsort Merge Cost: {} \
                        \n<br> Submission Size: {}",
                        submission.powersort_comp,
                        submission.timsort_comp,
                        submission.perc_diff,
                        submission.powersort_merge_cost,
                        submission.timsort_merge_cost,
                        submission.submission_size);

    mailer::send_email(body, usr_details.email.clone());
}

fn get_email_from_user_id(usr_id: i32) -> String
{
    let res = users.filter(user_id.eq(usr_id)).load::<User>(&mut init_db());
    
    return res.expect("Error loading user email!").get(0).unwrap().email.clone();
}

// Top 5 GLOBAL submissions irrespective of upload size.
#[get("/top_5_submissions")]
pub async fn top_5_submissions() -> HttpResponse
{
    let res = submissions.order(perc_diff.desc()).limit(5).load::<Submission>(&mut init_db());

    let mut top_5 = Vec::new();

    for submission in res.expect("Error loading top 5 submissions!")
    {
        top_5.push(submission);
    }

    HttpResponse::Ok().json(top_5)
}

// class = flyweight OR mediumweight OR heavyweight.
#[get("/weightclass_leading_submissions/{class}")]
pub async fn weightclass_leading_submissions(class: Path<String>) -> HttpResponse
{
    let _class = class.into_inner();
    println!("Getting top 5 submissions for weight class: {}", _class);

    let res;
    
    if (_class == "flyweight")
    {
        res = submissions
            .filter(submission_size.lt(i32::pow(10, 4)))
            .order(perc_diff.desc())
            .limit(5)
            .load::<Submission>(&mut init_db());
    }
    else if (_class == "mediumweight")
    {
        res = submissions
            .filter(submission_size.ge(i32::pow(10, 4)).and(submission_size.lt(i32::pow(10, 6))))
            .order(perc_diff.desc())
            .limit(5)
            .load::<Submission>(&mut init_db());
    }
    else // heavyweight
    {
        res = submissions
            .filter(submission_size.ge(i32::pow(10, 6)))
            .order(perc_diff.desc())
            .limit(5)
            .load::<Submission>(&mut init_db());
    }
    
    let mut top_5 = Vec::new();
    
    for submission in res.expect("Error loading top 5 flyweight submissions!")
    {
        top_5.push(submission);
    }
    
    HttpResponse::Ok().json(top_5)
}

#[allow(unused_parens)]
#[post("/logged_in")]
pub async fn login_probe(usr_details: Json<NewUser>) -> HttpResponse
{
    // Check if user already exists in the database via their email address.
    let usr_exists = check_usr_exists(init_db(), usr_details.email.clone());
    
    if (!usr_exists)
    {
        println!("Creating new user in database.");
        // Create a new user in the database.
        let _new_usr = NewUser
        {
            first_name: usr_details.first_name.clone(),
            last_name: usr_details.last_name.clone(),
            email: usr_details.email.clone()
        };

        let _ = diesel::insert_into(users).values(&_new_usr).execute(&mut init_db());
    }

    HttpResponse::Ok().json("Success".to_string())
}

#[post("/my_user_id")]
pub async fn my_user_id(usr_details: Json<User>) -> HttpResponse
{
    // We are only concerned with user email, as that is unique.
    let res = users.filter(email.eq(usr_details.email.clone())).load::<User>(&mut init_db());
    
    HttpResponse::Ok().json(res.expect("Error loading user ID!").get(0).unwrap().user_id)
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
        perc_diff: submission.perc_diff.clone(),
        powersort_merge_cost: submission.powersort_merge_cost.clone(),
        timsort_merge_cost: submission.timsort_merge_cost.clone(),
        submission_size: submission.submission_size.clone()
    };
    let s_id = diesel::insert_into(submissions).values(&_new_submission)
        .returning(submission_id)
        .get_result::<i32>(&mut init_db());

    // Send email receipt to user.
    let email_addr = get_email_from_user_id(_new_submission.user_id);
    dispatch_mail_receipt(Json(User { 
        user_id: 0, first_name: "".to_string(), last_name: "".to_string(), email: email_addr 
    }), _new_submission);
    
    HttpResponse::Ok().json(s_id.unwrap())
}

#[post("/submission_input_save")]
pub async fn submission_input_save(req: HttpRequest,
                                   MultipartForm(mut form): MultipartForm<FileDownload>) -> HttpResponse
{
    println!("Saving user submission input.");

    let headers = req.headers();
    let mut contents = String::new();

    let file_name = headers.get("file-name").unwrap().to_str().unwrap();
    let file_path = format!("{}/submissions/{}", env::var("EGRESS_DIR").unwrap(),
                                                                 file_name);

    form.file[0].file.read_to_string(&mut contents).expect("Error reading file contents!");

    // Save the file contents to the file system.
    fs::write(&file_path, &contents).expect("Unable to write file to disk!");

    HttpResponse::Ok().json("Success".to_string())
}

#[post("/serverside_calc")]
pub async fn serverside_calc(input: Json<String>) -> HttpResponse
{
    println!("Performing server-side calculation.");

    HttpResponse::Ok().json("Success".to_string())
}
