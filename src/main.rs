use actix_web::{get, post, Responder, web::{Json, Data}, HttpServer, App, HttpResponse, dev::Path};
use person_api::{models::CreatePerson, repository::Database};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_instance = Database::new();
    let app_data = Data::new(db_instance);
    HttpServer::new(move || {
        App::new()
        .app_data(app_data.clone())
        .service(get_all_persons)
        .service(create_person)
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}


#[get("/persons")]
async fn get_all_persons(db: Data<Database>) -> impl Responder {
    // Get all entries from db using diesel
    let result = db.get_persons();
    Json(result)
}


#[post("/persons")]
async fn create_person(mut create_person: Json<CreatePerson>, db: Data<Database>) -> impl Responder {
    // Insert data into db using diesel

    print!("Creating new person ...");
    db.insert_person(&mut create_person); 
    HttpResponse::Created()
}

