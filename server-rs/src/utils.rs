use crate::diesel::Connection;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub const DATE_FORMAT: &'static str = "%Y-%m-%dT%H:%M:%S%.3fZ";

pub fn get_env(key: &str) -> String {
    dotenv().ok();
    env::var(String::from(key)).expect(&format!("{} not present in `.env` file.", key))
}

pub fn establish_connection() -> PgConnection {
    let db_url = &get_env("DATABASE_URL").as_str();
    PgConnection::establish(db_url).expect(&format!("Error connecting to {}", db_url))
}
