use diesel::{Queryable, Identifiable, Associations, Selectable, Insertable, AsChangeset};
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct PersonCRUD {
    pub person: Person,
    pub document: Document,
    pub address: Address
}

#[derive(Serialize, Deserialize, Queryable, Selectable,  Identifiable, Associations,Insertable, Debug, PartialEq, AsChangeset)]
#[diesel(belongs_to(Person))]
#[diesel(primary_key(id))]
#[diesel(table_name = crate::schema::address)]
pub struct Address {
    #[serde(skip_deserializing)]
    #[diesel(deserialize_as = i32)]
    id: Option<i32>,
    state: String,
    city: String,
    country: String,
    zip_code: String,
    neighborhood: Option<String>,
    complement: Option<String>,
    number: String,
    #[serde(skip_deserializing)]
    person_id: i32
}


#[derive(Serialize, Deserialize, Debug)]
pub enum DocumentType {
    CPF(String),
    RG(String)
}


#[derive(Serialize, Deserialize, Queryable, Identifiable, Associations, Selectable, Insertable, Debug, PartialEq, AsChangeset)]
#[diesel(primary_key(id))]
#[diesel(belongs_to(Person))]
#[diesel(table_name = crate::schema::document)]
pub struct Document {
    #[serde(skip_deserializing)]
    #[diesel(deserialize_as = i32)]
    id: Option<i32>,
    doc_type: String,
    doc_number: String,
    #[serde(skip_deserializing)]
    person_id: i32
}



#[derive(Serialize, Deserialize, Queryable, Selectable, Identifiable, Insertable, PartialEq, Debug, AsChangeset)]
#[diesel(primary_key(id))]
#[diesel(table_name = crate::schema::person)]
pub struct Person {
    #[serde(skip_deserializing)]
    #[diesel(deserialize_as = i32)]
    id: Option<i32>,
    name: String,
    age: i32
}

impl Person {
   pub fn get_id(&self) -> i32{
       self.id.unwrap()
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
