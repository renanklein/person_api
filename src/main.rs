use actix_web::{get, post, Responder, web::Json, HttpServer, App, HttpResponse};
use person_api::{models::CreatePerson, repository::{insert_person, get_persons}};

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
    get_persons();
    Json("brah")
}

#[post("/persons")]
async fn create_person(mut create_person: Json<CreatePerson>) -> impl Responder {
    // Insert data into db using diesel
    insert_person(&mut create_person);
    HttpResponse::Created()
}

