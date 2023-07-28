use std::env;

use diesel::{PgConnection, Connection};

use crate::models::NewAddress;

pub fn establish_connection() -> PgConnection{
    let db_url = env::var("DATABASE_URL").expect("Env var 'DATABASE_URL' needs to be setted up");

    PgConnection::establish(&db_url).unwrap_or_else(|_| panic!("Error on trying to establish db connection"))
}


pub fn insert_address(address: NewAddress){
    println!("Inserting new address {:?}", address);
    use crate::schema::address::dsl::*;

    let mut connection = establish_connection();

    diesel::insert_into()
}