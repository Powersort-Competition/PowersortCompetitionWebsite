use std::env;
use std::fs;
use std::io::{Read, Write};

use tempfile::NamedTempFile;

use actix_multipart::form::MultipartForm;
use actix_web::web::Path;
use actix_web::{get, post, web::Json, HttpRequest, HttpResponse};
use diesel::prelude::*;
use futures::executor::block_on;

use crate::crypto::hash_submission;
use crate::database::init_db;
use crate::{mailer, python_hook};
use crate::models::{CompositionTrackA, FileDownload, NewSubmission, NewSubmission2, NewUser, Submission, SubmissionHash, SubmissionView, User};
use crate::schema::tracka_submission_hashes::dsl::tracka_submission_hashes;
use crate::schema::tracka_submissions::dsl::tracka_submissions;
use crate::schema::tracka_submissions::{mcost_diff, comp_diff, submission_id, submission_size};
use crate::schema::trackb_submissions::dsl::trackb_submissions;
use crate::schema::users::dsl::*;

#[get("/ping")]
pub async fn ping() -> HttpResponse {
    HttpResponse::Ok().json("pong".to_string())
}

/*
Internal function to check if user exists by probing database with email.
 */
#[allow(unused_parens)]
fn check_usr_exists(mut conn: PgConnection, email_addr: String) -> bool {
    let res = users.filter(email.eq(email_addr)).load::<User>(&mut conn);
    let res_size = res
        .expect("Error checking if user exists in database!")
        .len();

    if (res_size == 1) {
        return true;
    } else {
        return false;
    }
}

/*
Internal function to dispatch a mail receipt to a submitter.
 */
fn dispatch_mail_receipt(
    email_addr: String,
    submission_hash_str: String,
    submission: NewSubmission,
    track: String
) {
    println!("Hash submission string: {}", submission_hash_str);
   
    let mut body = String::new();
    if (track == "A")
    {
        body = format!(
            "Hello! Your submission for track A has been recorded successfully. \
                        \n\n<br> <br> Powersort Comparison: {} \
                        \n<br> Timsort Comparison: {} \
                        \n<br> Difference in comparisons (%): {} \
                        \n<br> Difference in merge costs (%): {} \
                        \n<br> Powersort Merge Cost: {} \
                        \n<br> Timsort Merge Cost: {} \
                        \n<br> Submission Size: {} \
                         \n\n<br> <br> <br> <br> <br> {} ",
            submission.powersort_comp,
            submission.timsort_comp,
            submission.comp_diff,
            submission.mcost_diff,
            submission.powersort_merge_cost,
            submission.timsort_merge_cost,
            submission.submission_size,
            submission_hash_str
        );
    }
    else // Track B.
    {
        body = format!(
            "Hello! Your submission for track B has been recorded successfully. \
                        \n\n<br> <br> <br> <br> <br> {} \
            ",
            submission_hash_str
        )
    }

    mailer::send_email(body, email_addr);
}

fn get_name_from_user_id(usr_id: i32) -> String {
    // Needs two returns. You cannot clone() a query result data type.
    // Maybe a future inventive way for one less DB call?
    let res_1 = users
        .filter(user_id.eq(usr_id))
        .load::<User>(&mut init_db());
    let res_2 = users
        .filter(user_id.eq(usr_id))
        .load::<User>(&mut init_db());

    let f_name = res_1
        .expect("Error loading user name!")
        .get(0)
        .unwrap()
        .first_name
        .clone();
    let l_name = res_2
        .expect("Error loading user name!")
        .get(0)
        .unwrap()
        .last_name
        .clone();

    return format!("{} {}.",
            f_name,
            l_name.chars().nth(0).unwrap());
}

fn get_email_from_user_id(usr_id: i32) -> String {
    let res = users
        .filter(user_id.eq(usr_id))
        .load::<User>(&mut init_db());

    return res
        .expect("Error loading user email!")
        .get(0)
        .unwrap()
        .email
        .clone();
}

// Top 5 GLOBAL submissions irrespective of upload size.
#[get("/top_5_submissions")]
pub async fn top_5_submissions() -> HttpResponse {
    let res = tracka_submissions
        .limit(5)
        .load::<Submission>(&mut init_db());

    let mut top_5 = Vec::new();

    for submission in res.expect("Error loading top 5 submissions!") {
        top_5.push(SubmissionView {
            submission_id: submission.submission_id,
            submitter: get_name_from_user_id(submission.user_id),
            user_id: submission.user_id,
            powersort_comp: submission.powersort_comp,
            timsort_comp: submission.timsort_comp,
            comp_diff: submission.comp_diff,
            mcost_diff: submission.mcost_diff,
            powersort_merge_cost: submission.powersort_merge_cost,
            timsort_merge_cost: submission.timsort_merge_cost,
            submission_size: submission.submission_size,

        });
    }

    HttpResponse::Ok().json(top_5)
}

#[get("/composition_track_a")]
pub async fn composition_track_a() -> HttpResponse {
    let mut conn = init_db();

    let flyweight_count = tracka_submissions
        .filter(submission_size.lt(i32::pow(10, 4)))
        .count()
        .get_result::<i64>(&mut conn)
        .unwrap();
    
    let mediumweight_count = tracka_submissions
        .filter(
            submission_size.ge(i32::pow(10, 4))
                .and(submission_size.lt(i32::pow(10, 6))))
        .count()
        .get_result::<i64>(&mut conn)
        .unwrap();

    let heavyweight_count = tracka_submissions
        .filter(submission_size.ge(i32::pow(10, 6)))
        .count()
        .get_result::<i64>(&mut conn)
        .unwrap();

    let counts = CompositionTrackA {
        flyweight: flyweight_count,
        mediumweight: mediumweight_count,
        heavyweight: heavyweight_count,
    };

    HttpResponse::Ok().json(counts)
}

// class = flyweight OR mediumweight OR heavyweight.
#[allow(unused_parens)]
#[get("/weightclass_leading_submissions/{class}")]
pub async fn weightclass_leading_submissions(req: HttpRequest, class: Path<String>) -> HttpResponse {
    let _class = class.into_inner();
    
    let headers = req.headers();
    let _orderBy = headers.get("order-by").unwrap().to_str().unwrap();
    let res;

    if (_class == "flyweight") {
        if (_orderBy == "mcost_diff") {
            res = tracka_submissions
                .filter(submission_size.lt(i32::pow(10, 4)))
                .order(mcost_diff.desc())
                .limit(5)
                .load::<Submission>(&mut init_db());
        } else // Order by ncomp_diff.
        {
           res = tracka_submissions
                .filter(submission_size.lt(i32::pow(10, 4)))
                .order(comp_diff.desc())
                .limit(5)
                .load::<Submission>(&mut init_db());

        }
    } else if (_class == "mediumweight") {
        if (_orderBy == "mcost_diff") {
            res = tracka_submissions
                .filter(
                    submission_size
                        .ge(i32::pow(10, 4))
                        .and(submission_size.lt(i32::pow(10, 6))),
                )
                .order(mcost_diff.desc())
                .limit(5)
                .load::<Submission>(&mut init_db());
        } else
        {
             res = tracka_submissions
                .filter(
                    submission_size
                        .ge(i32::pow(10, 4))
                        .and(submission_size.lt(i32::pow(10, 6))),
                )
                .order(comp_diff.desc())
                .limit(5)
                .load::<Submission>(&mut init_db());
        }
    } else
    // heavyweight
    {
        if (_orderBy == "mcost_diff") {
            res = tracka_submissions
                .filter(submission_size.ge(i32::pow(10, 6)))
                .order(mcost_diff.desc())
                .limit(5)
                .load::<Submission>(&mut init_db());
        } else
        {
            res = tracka_submissions
                .filter(submission_size.ge(i32::pow(10, 6)))
                .order(comp_diff.desc())
                .limit(5)
                .load::<Submission>(&mut init_db());
        }
    }

    let mut top_5 = Vec::new();

    for submission in res.expect("Error loading top 5 flyweight submissions!") {
        top_5.push(SubmissionView {
            submission_id: submission.submission_id,
            submitter: get_name_from_user_id(submission.user_id),
            user_id: submission.user_id,
            powersort_comp: submission.powersort_comp,
            timsort_comp: submission.timsort_comp,
            comp_diff: submission.comp_diff,
            mcost_diff: submission.mcost_diff,
            powersort_merge_cost: submission.powersort_merge_cost,
            timsort_merge_cost: submission.timsort_merge_cost,
            submission_size: submission.submission_size
        });
    }

    HttpResponse::Ok().json(top_5)
}

#[allow(unused_parens)]
#[post("/logged_in")]
pub async fn login_probe(usr_details: Json<NewUser>) -> HttpResponse {
    // Check if user already exists in the database via their email address.
    let usr_exists = check_usr_exists(init_db(), usr_details.email.clone());

    if (!usr_exists) {
        println!("Creating new user in database.");
        // Create a new user in the database.
        let _new_usr = NewUser {
            first_name: usr_details.first_name.clone(),
            last_name: usr_details.last_name.clone(),
            email: usr_details.email.clone(),
        };

        let _ = diesel::insert_into(users)
            .values(&_new_usr)
            .execute(&mut init_db());
    }

    HttpResponse::Ok().json("Success".to_string())
}

#[post("/my_user_id")]
pub async fn my_user_id(usr_details: Json<User>) -> HttpResponse {
    // We are only concerned with user email, as that is unique.
    let res = users
        .filter(email.eq(usr_details.email.clone()))
        .load::<User>(&mut init_db());

    HttpResponse::Ok().json(res.expect("Error loading user ID!").get(0).unwrap().user_id)
}

#[post("/new_submission_track_a")]
pub async fn new_submission_track_a(submission: Json<NewSubmission>) -> HttpResponse {
    println!("Adding new user submission to database.");

    let _new_submission = NewSubmission {
        user_id: submission.user_id,
        powersort_comp: submission.powersort_comp.clone(),
        timsort_comp: submission.timsort_comp.clone(),
        comp_diff: submission.comp_diff.clone(),
        mcost_diff: submission.mcost_diff.clone(),
        powersort_merge_cost: submission.powersort_merge_cost.clone(),
        timsort_merge_cost: submission.timsort_merge_cost.clone(),
        submission_size: submission.submission_size.clone(),
    };
    let s_id = diesel::insert_into(tracka_submissions)
        .values(&_new_submission)
        .returning(submission_id)
        .get_result::<i32>(&mut init_db())
        .unwrap();

    // Hash submission ID, and record it to the database. Then, send it via email.
    let submission_hash_str = block_on(hash_submission(s_id));
    let _submission_hash = SubmissionHash {
        submission_id: s_id,
        hash: submission_hash_str.clone(),
    };

    let _ = diesel::insert_into(tracka_submission_hashes)
        .values(&_submission_hash)
        .execute(&mut init_db())
        .unwrap();

    // Send email receipt to user.
    let email_addr = get_email_from_user_id(_new_submission.user_id);
    dispatch_mail_receipt(email_addr, submission_hash_str, _new_submission, "A".to_string());

    HttpResponse::Ok().json(s_id)
}

// TODO: If not too much work, merge with new_submission_track_a.
#[post("/new_submission_track_b")]
pub async fn new_submission_track_b(submission: Json<NewSubmission2>) -> HttpResponse {
    println!("Adding new user submission to database.");
    
    let _new_submission = NewSubmission2 {
        user_id: submission.user_id,
    };
    
    let s_id = diesel::insert_into(trackb_submissions)
        .values(&_new_submission)
        .returning(crate::schema::trackb_submissions::submission_id)
        .get_result::<i32>(&mut init_db())
        .unwrap();
   
    let submission_hash_str = block_on(hash_submission(s_id));
    
    // Send email receipt to user.
    let email_addr = get_email_from_user_id(_new_submission.user_id);
    dispatch_mail_receipt(email_addr,
                          submission_hash_str,
                          NewSubmission { // Dummy data.
                              user_id: 0,
                              powersort_comp: 0,
                              timsort_comp: 0,
                              comp_diff: 0.0,
                              mcost_diff: 0.0,
                              powersort_merge_cost: 0,
                              timsort_merge_cost: 0,
                              submission_size: 0,
                          },
                          "B".to_string());
    
    HttpResponse::Ok().json(s_id)
}

#[post("/submission_input_save")]
pub async fn submission_input_save(
    req: HttpRequest,
    MultipartForm(mut form): MultipartForm<FileDownload>,
) -> HttpResponse {
    println!("Saving user submission input.");

    let headers = req.headers();
    let mut contents = String::new();

    let file_name = headers.get("file-name").unwrap().to_str().unwrap();
    let track = headers.get("track").unwrap().to_str().unwrap();
    println!("Track: {}", track);

    let file_path;
    if (track == "A")
    // Track A
    {
        file_path = format!(
            "{}/submissions/TrackA/{}",
            env::var("EGRESS_DIR").unwrap(),
            file_name
        );
        
        form.file[0]
            .file
            .read_to_string(&mut contents)
            .expect("Error reading file contents!");
    } else
    // Track B
    {
        // Special handling is required for Track B submissions, as we have descriptions too.
        let mut input_data = String::new();
        let mut description = String::new();
        
        file_path = format!(
            "{}/submissions/TrackB/{}",
            env::var("EGRESS_DIR").unwrap(),
            file_name
        );

        form.file[0]
            .file
            .read_to_string(&mut input_data)
            .expect("Error reading file contents!");
        
        description = headers.get("description").unwrap().to_str().unwrap().parse().unwrap();
        
        // Concatenate the input data and description into contents variable.
        contents = format!("{}\n\n{}", input_data, description);
    }

    // Save the file contents to the file system.
    fs::write(&file_path, &contents).expect("Unable to write file to disk!");

    HttpResponse::Ok().json("Success".to_string())
}

#[post("/serverside_calc")]
pub async fn serverside_calc(req: HttpRequest,
                             MultipartForm(mut form): MultipartForm<FileDownload>
) -> HttpResponse {
    println!("Performing server-side calculation.");
    
    let mut input_data = String::new();
    let mut usr_id: i32;
    
    usr_id = req.headers().get("user-id").unwrap().to_str().unwrap().parse().unwrap();
    form.file[0]
        .file
        .read_to_string(&mut input_data)
        .expect("Error reading file contents!");

    // Create a temporary file on the filesystem containing input_data.
    let mut temp_file = NamedTempFile::new().expect("Error creating temporary file!"); 
    writeln!(temp_file, "{}", input_data).expect("Error writing to temporary file!");
    // No weird unicode characters, so safe to use lossy string conversion.
    let temp_file_path = temp_file.path().to_path_buf().to_string_lossy().to_string();
   
    // Call Python hook for the computation. Wait till computation is done.
    let computation_result = block_on(python_hook::run_python_script(temp_file_path));
    
    HttpResponse::Ok().json("Success".to_string())
}
