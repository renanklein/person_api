use actix_web::{get, post, Responder, web::{Json, Data, Path}, HttpServer, App, HttpResponse, put, delete};
use person_api::{models::PersonCRUD, repository::Database};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_instance = Database::new();
    let app_data = Data::new(db_instance);
    HttpServer::new(move || {
        App::new()
        .app_data(app_data.clone())
        .service(get_all_persons)
        .service(create_person)
        .service(get_persons_by_id)
        .service(update_person)
        .service(delete_person)
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

#[get("/persons/{id}")]
async fn get_persons_by_id(id_path: Path<i32>, db: Data<Database>) -> impl Responder {
    let result = db.get_person_by_id(id_path.into_inner());

    Json(result)
}

#[post("/persons")]
async fn create_person(mut create_person: Json<PersonCRUD>, db: Data<Database>) -> impl Responder {
    // Insert data into db using diesel
    print!("Creating new person ...");
    
    db.insert_person(&mut create_person); 
    HttpResponse::Created()
}

#[put("/persons/{id}")]
async fn update_person(update_person: Json<PersonCRUD>, db: Data<Database>, p_id: Path<i32>) -> impl Responder {
    db.update_person(p_id.into_inner(), update_person.into_inner());

    HttpResponse::Ok()
}

#[delete("/persons/{id}")]
async fn delete_person(p_id: Path<i32>, db: Data<Database>) -> impl Responder{
    db.delete_person(p_id.into_inner());

    HttpResponse::Ok()
}