use actix_web::{HttpServer, App};
use listenfd;
use dotenv::dotenv;
use std::env;


mod utils;
mod database;
mod api;

#[actix_rt::main]
async fn main() -> std::io::Result<()>
{
    // Initialise the logger.
    env_logger::init();
    // Initialise connection to the database.
    //database::init();
    // Get (static) variables from .env file.
    dotenv().ok();

    let mut listenfd = listenfd::ListenFd::from_env();
    let mut actix_server = HttpServer::new(move
        || App::new().service(
        api::usr_authenticated
    ));
    actix_server = match listenfd.take_tcp_listener(0)?
    {
        Some(listener) => actix_server.listen(listener)?,
        None => {
            let host = env::var("BIND_IP").unwrap();
            let port = env::var("BIND_PORT").unwrap();

            actix_server.bind(format!("{}:{}", host, port))?
        }
    };

    actix_server.run().await
}
