use std::env;

use diesel::{ExpressionMethods, JoinOnDsl, PgConnection, QueryDsl, RunQueryDsl, r2d2::{ConnectionManager, Pool}};

use crate::{
    models::{Address, CreatePerson, Document, Person},
    schema::{address, document},
};

type PgPool = Pool<ConnectionManager<PgConnection>>;

pub struct Database {
    pool: PgPool,
}

impl Database {
    pub fn new() -> Self{
        let db_url =
            env::var("DATABASE_URL").expect("Env var 'DATABASE_URL' needs to be setted up");

        let manager = ConnectionManager::<PgConnection>::new(db_url);

        let pool_result = Pool::builder().build(manager);

        let pool = match pool_result {
            Ok(pool) => pool,
            Err(err) => panic!("Error on creating pool of connections{:?}", err),
        };

        Database { pool }
    }
    
    fn insert_address(&self, new_address: &Address) {
        println!("Inserting new address {:?}", new_address);
        use crate::schema::address::dsl::*;


        //new_address.set_person_id(inserted_person_id);

        diesel::insert_into(address)
            .values(new_address)
            .execute(&mut self.pool.get().unwrap())
            .expect("An error occur trying to insert record");
    }

    fn insert_document(&self, new_doc: &Document) {
        println!("Inserting new document {:?}", new_doc);

        use crate::schema::document::dsl::*;

        diesel::insert_into(document)
            .values(new_doc)
            .execute(&mut self.pool.get().unwrap())
            .expect("Error on inserting documents");
    }

    pub fn insert_person(&self, new_person: &mut CreatePerson) {
        println!("Inserting new person {:?}", new_person);

        use crate::schema::person::dsl::*;

        let inserted_record = diesel::insert_into(person)
            .values(&new_person.person)
            .get_result::<Person>(&mut self.pool.get().unwrap())
            .expect("Error on inserting new person");

        new_person.address.set_person_id(&inserted_record.get_id());
        new_person.document.set_person_id(&inserted_record.get_id());

        self.insert_address(&new_person.address);
        self.insert_document(&new_person.document);
    }

    pub fn get_persons(&self) -> Vec<(Person, Address, Document)> {
        println!("Getting all persons ...");

        use crate::schema::person::dsl::*;

        person
            .inner_join(address::table)
            .inner_join(document::table)
            .load::<(Person, Address, Document)>(&mut self.pool.get().unwrap())
            .unwrap()
    }

    pub fn get_person_by_id(&self, p_id: i32) -> (Person, Address, Document) {
        println!("Getting person by id ...");
        use crate::schema::person::dsl::*;

        person
            .inner_join(address::table.on(address::person_id.eq(p_id)))
            .inner_join(document::table.on(document::person_id.eq(p_id)))
            .get_result::<(Person, Address, Document)>(&mut self.pool.get().unwrap())
            .unwrap()
    }
}
