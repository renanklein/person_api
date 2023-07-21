use std::env;

use diesel::{PgConnection, Connection};

pub mod models;
pub mod schema;
pub mod repository;


pub fn establish_connection() -> PgConnection{
    let db_url = env::var("DATABASE_URL").expect("Env var 'DATABASE_URL' needs to be setted up");

    PgConnection::establish(&db_url).unwrap_or_else(|_| panic!("Error on trying to establish db connection"))
}