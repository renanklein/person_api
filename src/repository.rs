use std::env;

use diesel::{Connection, PgConnection, RunQueryDsl,QueryDsl};

use crate::{
    models::{NewAddress, NewDocument, Person, CreatePerson},
    schema::{address, document},
};

pub fn establish_connection() -> PgConnection {
    let db_url = env::var("DATABASE_URL").expect("Env var 'DATABASE_URL' needs to be setted up");

    PgConnection::establish(&db_url)
        .unwrap_or_else(|_| panic!("Error on trying to establish db connection"))
}

fn insert_address(new_address: &NewAddress) {
    println!("Inserting new address {:?}",new_address);
    use crate::schema::address::dsl::*;

    let connection = &mut establish_connection();

    diesel::insert_into(address)
        .values(new_address)
        .execute(connection)
        .expect("An error occur trying to insert record");
}

fn insert_document(new_doc: &NewDocument) {
    println!("Inserting new document {:?}", new_doc);

    use crate::schema::document::dsl::*;

    let connection = &mut establish_connection();

    diesel::insert_into(document)
        .values(new_doc)
        .execute(connection)
        .expect("Error on inserting documents");
}

pub fn insert_person(new_person: &mut CreatePerson) {
    println!("Inserting new person {:?}", new_person);

    use crate::schema::person::dsl::*;

    let connection = &mut establish_connection();

    let inserted_record =  diesel::insert_into(person)
        .values(&new_person.person)
        .get_result::<Person>(connection)
        .expect("Error on inserting new person");

   new_person.address.set_person_id(&inserted_record.get_id());
   new_person.document.set_person_id(&inserted_record.get_id());

    insert_address(&new_person.address);
    insert_document(&new_person.document);
}

pub fn get_persons(){
    println!("Getting all persons ...");

    use crate::schema::person::dsl::*;

    let connection = &mut establish_connection();

    let result = person
        .inner_join(address::table)
        .inner_join(document::table)
        .execute(connection);

    println!("Results from query {:?}", result);
}
