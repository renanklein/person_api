use actix_web::{get, post, patch, delete, Responder, web::Json, HttpServer, App, HttpResponse};
use person_api::models::Person;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .service(get_all_persons)
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}


#[get("/persons")]
async fn get_all_persons() -> impl Responder {
    // Get all entries from db using diesel
    Json("brah")
}

#[post("/persons")]
async fn create_person(person: Json<Person>) -> impl Responder {
    // Insert data into db using diesel

    HttpResponse::Created()
}

