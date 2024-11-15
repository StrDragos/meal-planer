use std::net::TcpListener;
use std::sync::Arc;
use crate::dishes::routes::dish_routes;
use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use tokio::sync::Mutex;
use crate::common::firebase::{Firebase, UserData};
use crate::config::AppConfig;

mod dishes;
pub mod config;
pub mod middlwares;
pub mod common;

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

pub struct AppData {
    pub user_data: Mutex<Option<UserData>>
}

pub fn run(listener: TcpListener, app_config: AppConfig) -> Result<Server, std::io::Error> {
    let firebase = Arc::new(Firebase { settings: app_config.firebase_settings.clone() });

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(
                web::scope("/api")
                    .route("/health_check", web::get().to(health_check))
                    .service(dish_routes(firebase.clone()))
            )
    })
        .listen(listener)?
        .run();

    Ok(server)
}