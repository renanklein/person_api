use diesel::{Queryable, Identifiable, Associations, Selectable};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Queryable, Identifiable, Associations, Debug)]
#[diesel(belongs_to(Person))]
#[diesel(table_name = crate::schema::address)]
pub struct Address {
    id: i32,
    state: String,
    city: String,
    country: String,
    zip_code: String,
    neighborhood: String,
    complement: String,
    street: String,
    number: String,
    person_id: i32
}


#[derive(Serialize, Deserialize, Debug)]
pub enum DocumentType {
    CPF(String),
    RG(String)
}


#[derive(Serialize, Deserialize, Queryable, Identifiable, Associations, Debug)]
#[diesel(belongs_to(Person))]
#[diesel(table_name = crate::schema::document)]
pub struct Document {
    id: i32,
    document_type: DocumentType,
    doc_number: String,
    person_id: i32
}



#[derive(Serialize, Deserialize, Queryable)]
#[diesel(table_name = crate::schema::person)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Person {
    id: String,
    name: String,
    age: i32,
    address: Address,
    document: Document
}

impl Person {
   pub fn get(self) -> Person{
       self 
    } 
}
