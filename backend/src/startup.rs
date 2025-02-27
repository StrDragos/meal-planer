use crate::common::firebase::{Context, Firebase};
use crate::config::AppConfig;
use crate::userdata::http::route::user_settings_routes;
use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;
use std::sync::Arc;
use tokio::sync::Mutex;

pub mod common;
pub mod config;
pub mod middlwares;
pub mod userdata;

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

pub struct AppData {
    pub user_data: Mutex<Option<Context>>,
}

pub async fn run(listener: TcpListener, app_config: AppConfig) -> Result<Server, std::io::Error> {
    let firebase = Arc::new(Firebase {
        settings: app_config.firebase_settings.clone(),
    });

    let db_pool = Arc::new(
        PgPoolOptions::new()
        .connect(&app_config.db_config.postgres_url())
        .await.expect("Failed to connect to db")
    );

    let server = HttpServer::new(move || {
        App::new().wrap(Logger::default()).service(
            web::scope("/api")
                .service(user_settings_routes(
                    firebase.clone(),
                    db_pool.clone(),
                ))
                .route("/health_check", web::get().to(health_check))
        )
    })
    .listen(listener)?
    .run();

    Ok(server)
}
