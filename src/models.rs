use diesel::{Queryable, Identifiable, Associations, Selectable, Insertable};
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct CreatePerson {
    pub person: NewPerson,
    pub document: NewDocument,
    pub address: NewAddress
}

#[derive(Serialize, Deserialize, Insertable, Debug)]
#[diesel(table_name = crate::schema::person)]
pub struct NewPerson {
    name: String,
    age: i32
}


#[derive(Serialize, Deserialize,Insertable, Debug, Clone)]
#[diesel(table_name = crate::schema::address)]
pub struct NewAddress {
    state: String,
    city: String,
    country: String,
    zip_code: String,
    neighborhood: Option<String>,
    complement: Option<String>,
    number: String
}


#[derive(Serialize, Deserialize, Insertable, Debug, Clone)]
#[diesel(table_name = crate::schema::document)]
pub struct NewDocument{
    doc_type: String,
    doc_number: String
}

#[derive(Queryable, Selectable,  Identifiable, Associations, Debug, PartialEq)]
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
    number: String,
    person_id: i32
}


#[derive(Serialize, Deserialize, Debug)]
pub enum DocumentType {
    CPF(String),
    RG(String)
}


#[derive(Serialize, Deserialize, Queryable, Identifiable, Associations, Selectable, Debug, PartialEq)]
#[diesel(belongs_to(Person))]
#[diesel(table_name = crate::schema::document)]
pub struct Document {
    id: i32,
    doc_type: String,
    doc_number: String,
    person_id: i32
}



#[derive(Serialize, Deserialize, Queryable, Selectable, Identifiable, PartialEq)]
#[diesel(table_name = crate::schema::person)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Person {
    id: i32,
    name: String,
    age: i32
}

impl Person {
   pub fn get_id(&self) -> i32{
       self.id
    } 
}
