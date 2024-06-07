use std::env;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

use dotenv::dotenv;

pub fn send_email(body: String, email: String)
{
    dotenv().ok();

    let mail = body_constructor(body, email);
    let username = env::var("MAIL_USER").expect("MAIL_USER must be set");
    let password = env::var("MAIL_PASS").expect("MAIL_PASS must be set");
    let server = env::var("MAIL_HOST").expect("MAIL_HOST be set");

    let creds = Credentials::new(username, password);
    let mailer = SmtpTransport::starttls_relay(&*server)
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email!
    match mailer.send(&mail)
    {
        Ok(_) => println!("An email has been sent!"),
        Err(e) => panic!("Could not send email! Error: {e:?}")
    }
}

fn body_constructor(body: String, email: String) -> Message
{
    let mail = Message::builder()
        .from(env::var("MAIL_FROM").unwrap().parse().unwrap())
        .to(email.parse().unwrap())
        .subject("Powersort Competition Receipt")
        .header(ContentType::TEXT_HTML)
        .body(body)
        .unwrap();

    return mail;
}