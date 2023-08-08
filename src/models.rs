use diesel::{Queryable, Identifiable, Associations, Selectable, Insertable};
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct CreatePerson {
    pub person: Person,
    pub document: Document,
    pub address: Address
}

#[derive(Serialize, Deserialize, Queryable, Selectable,  Identifiable, Associations,Insertable, Debug, PartialEq)]
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
    #[serde(skip_serializing)]
    person_id: i32
}


#[derive(Serialize, Deserialize, Debug)]
pub enum DocumentType {
    CPF(String),
    RG(String)
}


#[derive(Serialize, Deserialize, Queryable, Identifiable, Associations, Selectable, Insertable, Debug, PartialEq)]
#[diesel(belongs_to(Person))]
#[diesel(table_name = crate::schema::document)]
pub struct Document {
    id: i32,
    doc_type: String,
    doc_number: String,
    #[serde(skip_serializing)]
    person_id: i32
}



#[derive(Serialize, Deserialize, Queryable, Selectable, Identifiable, Insertable, PartialEq, Debug)]
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

impl Address {
    pub fn set_person_id(&mut self, person_id: &i32){
        self.person_id = *person_id;
    }
}

impl Document {
    pub fn set_person_id(&mut self, person_id: &i32){
        self.person_id = *person_id;
    }
}
