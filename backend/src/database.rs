use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use lazy_static::lazy_static;
use dotenv::dotenv;
use std::env;
use diesel_migrations::embed_migrations;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
lazy_static!
{
    pub static ref POOL: Pool =
    {
        let db_ip = env::var("DB_IP").unwrap();
        let db_port = env::var("DB_PORT").unwrap();

        let db_manager = ConnectionManager::<PgConnection>::new(db_ip);
        let pool: Pool = r2d2::Pool::builder().build(db_manager).expect("db_manager failed!");

        pool
    };
}

pub fn init()
{
    dotenv().ok();

    lazy_static::initialize(&POOL);
    let conn = POOL.get();

}


