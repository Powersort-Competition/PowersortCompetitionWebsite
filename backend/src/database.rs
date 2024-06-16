use std::env;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;

pub fn init_db() -> PgConnection {
    dotenv().ok();

    let db_user = env::var("DB_USER").expect("DB_USER must be set");
    let db_password = env::var("DB_PASSWORD").expect("DB_PASSWORD must be set");
    let db_name = env::var("DB_NAME").expect("DB_NAME must be set");
    let db_host = env::var("DB_IP").expect("DB_IP must be set");
    let db_port = env::var("DB_PORT").expect("DB_PORT must be set");

    let db_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        db_user, db_password, db_host, db_port, db_name
    );

    PgConnection::establish(&db_url).unwrap_or_else(|_| panic!("Error connecting to database!"))
}
