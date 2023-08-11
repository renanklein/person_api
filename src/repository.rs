use std::env;

use diesel::{Connection, ExpressionMethods, JoinOnDsl, PgConnection, QueryDsl, RunQueryDsl};

use crate::{
    models::{Address, CreatePerson, Document, Person},
    schema::{address, document},
};

pub struct Database {
    connection: PgConnection,
}

impl Database {
    pub fn new() -> Self{
        let db_url =
            env::var("DATABASE_URL").expect("Env var 'DATABASE_URL' needs to be setted up");

        let connection_result = PgConnection::establish(&db_url).unwrap_or_else(|err| -> {panic!("Error on creating connection {:?}", err)});

        match connection_result {
            Ok(conn) => conn,
            Err(err) => panic!("Error on creating connection {:?}", err),
        }

        Database { connection: conn }
    }
    fn establish_connection(&self) -> PgConnection {
        let db_url =
            env::var("DATABASE_URL").expect("Env var 'DATABASE_URL' needs to be setted up");

        let connection_result = PgConnection::establish(&db_url);

        match connection_result {
            Ok(conn) => conn,
            Err(err) => panic!("Error on creating connection {:?}", err),
        }
    }

    fn insert_address(&self, new_address: &Address) {
        println!("Inserting new address {:?}", new_address);
        use crate::schema::address::dsl::*;

        let connection = &mut establish_connection();

        //new_address.set_person_id(inserted_person_id);

        diesel::insert_into(address)
            .values(new_address)
            .execute(connection)
            .expect("An error occur trying to insert record");
    }

    fn insert_document(&self, new_doc: &Document) {
        println!("Inserting new document {:?}", new_doc);

        use crate::schema::document::dsl::*;

        let connection = &mut establish_connection();

        diesel::insert_into(document)
            .values(new_doc)
            .execute(connection)
            .expect("Error on inserting documents");
    }

    pub fn insert_person(&self, new_person: &mut CreatePerson) {
        println!("Inserting new person {:?}", new_person);

        use crate::schema::person::dsl::*;

        let connection = &mut establish_connection();

        let inserted_record = diesel::insert_into(person)
            .values(&new_person.person)
            .get_result::<Person>(connection)
            .expect("Error on inserting new person");

        new_person.address.set_person_id(&inserted_record.get_id());
        new_person.document.set_person_id(&inserted_record.get_id());

        insert_address(&new_person.address);
        insert_document(&new_person.document);
    }

    pub fn get_persons(&self) -> Vec<(Person, Address, Document)> {
        println!("Getting all persons ...");

        use crate::schema::person::dsl::*;

        let connection = &mut establish_connection();

        person
            .inner_join(address::table)
            .inner_join(document::table)
            .load::<(Person, Address, Document)>(connection)
            .unwrap()
    }

    pub fn get_person_by_id(&self, p_id: i32) -> (Person, Address, Document) {
        println!("Getting person by id ...");
        use crate::schema::person::dsl::*;

        let connection = &mut establish_connection();

        person
            .inner_join(address::table.on(address::person_id.eq(p_id)))
            .inner_join(document::table.on(document::person_id.eq(p_id)))
            .get_result::<(Person, Address, Document)>(connection)
            .unwrap()
    }
}
