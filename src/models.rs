use diesel::{Queryable, Selectable, Insertable};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Queryable,Debug)]
struct Address {
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
enum DocumentType {
    CPF(String),
    RG(String)
}


#[derive(Serialize, Deserialize, Queryable,Debug)]
struct Document {
    document_type: DocumentType,
    doc_number: String
}



#[derive(Serialize, Deserialize,Debug)]
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
