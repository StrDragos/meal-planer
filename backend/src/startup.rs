use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_web::dev::Server;
use actix_web::middleware::Logger;
use crate::config::AppConfig;
use crate::dishes::routes::dish_routes;

mod dishes;
pub mod config;
mod middlwares;


async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

pub fn run(app_config: &AppConfig) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(
            web::scope("/api")
                .route("health_check", web::get().to(health_check))
                .service(dish_routes())
            )
    })
        .bind(("127.0.0.1", app_config.port.unwrap_or(8080)))?
        .run();

    Ok(server)
}