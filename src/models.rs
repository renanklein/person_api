use diesel::{Queryable, Selectable, Insertable};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Queryable,Debug)]
pub struct Address {
    state: String,
    city: String,
    country: String,
    zip_code: String,
    neighborhood: String,
    complement: String,
    street: String,
    number: String
}


#[derive(Serialize, Deserialize, Debug)]
pub enum DocumentType {
    CPF(String),
    RG(String)
}


#[derive(Serialize, Deserialize, Queryable, Debug)]
pub struct Document {
    document_type: DocumentType,
    doc_number: String
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
