use actix_web::{HttpServer, App};
use listenfd;
use dotenv::dotenv;
use std::env;


mod utils;
mod api;
mod schema;
mod models;
mod database;

#[actix_rt::main]
async fn main() -> std::io::Result<()>
{
    // Initialise the logger.
    env::set_var("RUST_LOG", "actix_web = info");
    env_logger::init();
    // Get (static) variables from .env file.
    dotenv().ok();

    let mut listenfd = listenfd::ListenFd::from_env();
    let mut actix_server = HttpServer::new(move
        || App::new().service(api::ping)
                     .service(api::login_probe));

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
