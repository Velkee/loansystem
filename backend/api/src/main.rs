use self::models::*;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use loansystem_backend::*;

#[get("/devices")]
async fn list_devices() -> impl Responder {
    use self::schema::devices::dsl::*;

    let connection = &mut establish_connection().await;
    let results: Vec<Device> = match devices.select(Device::as_select()).load(connection).await {
        Ok(results) => results,
        Err(_) => {
            return HttpResponse::InternalServerError()
                .body("500: Internal server error\nDatabase unavailable or broken.")
        }
    };

    HttpResponse::Ok().json(results)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(list_devices))
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}
