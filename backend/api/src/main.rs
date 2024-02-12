use self::models::*;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use loansystem_backend::*;

async fn internal_error() -> HttpResponse {
    HttpResponse::InternalServerError().body("")
}

#[get("/devices")]
async fn list_devices() -> impl Responder {
    use self::schema::devices::dsl::*;

    let connection = &mut establish_connection().await;
    let results: Vec<Device> = match devices.select(Device::as_select()).load(connection).await {
        Ok(results) => results,
        Err(_) => return internal_error().await,
    };

    println!("{:#?}", results);

    HttpResponse::Ok().json(results)
}

#[get("/categories")]
async fn list_categories() -> impl Responder {
    use self::schema::categories::dsl::*;

    let connection = &mut establish_connection().await;
    let results: Vec<Category> = match categories
        .select(Category::as_select())
        .load(connection)
        .await
    {
        Ok(results) => results,
        Err(_) => return internal_error().await,
    };

    HttpResponse::Ok().json(results)
}

#[get("/people")]
async fn list_people() -> impl Responder {
    use self::schema::people::dsl::*;

    let connection = &mut establish_connection().await;
    let results: Vec<Person> = match people.select(Person::as_select()).load(connection).await {
        Ok(results) => results,
        Err(_) => return internal_error().await,
    };

    HttpResponse::Ok().json(results)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(list_devices)
            .service(list_categories)
            .service(list_people)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
