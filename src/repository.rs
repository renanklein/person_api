use std::env;

use diesel::{connection, Connection, PgConnection, RunQueryDsl};

use crate::{
    models::{NewAddress, NewDocument, NewPerson},
    schema,
};

pub fn establish_connection() -> PgConnection {
    let db_url = env::var("DATABASE_URL").expect("Env var 'DATABASE_URL' needs to be setted up");

    PgConnection::establish(&db_url)
        .unwrap_or_else(|_| panic!("Error on trying to establish db connection"))
}

fn insert_address(new_address: NewAddress) {
    println!("Inserting new address {:?}", new_address);
    use crate::schema::address::dsl::*;

    let connection = &mut establish_connection();

    diesel::insert_into(address)
        .values(&new_address)
        .execute(connection)
        .expect("An error occur trying to insert record");
}

fn insert_document(new_doc: NewDocument) {
    println!("Inserting new document {:?}", new_doc);

    use crate::schema::document::dsl::*;

    let connection = &mut establish_connection();

    diesel::insert_into(document)
        .values(&new_doc)
        .execute(connection)
        .expect("Error on inserting documents");
}

pub fn insert_person(new_person: NewPerson, new_address: NewAddress, new_doc: NewDocument) {
    println!("Inserting new person {:?}", new_person);

    use crate::schema::person::dsl::*;

    let connection = &mut establish_connection();

    diesel::insert_into(person)
        .values(&new_person)
        .execute(connection)
        .expect("Error on inserting new person");

    insert_address(new_address);
    insert_document(new_doc);
}
