use crate::common::firebase::Firebase;
use crate::middlwares::authorization::authorize;
use crate::userdata::http::data::user_data::UserData;
use crate::userdata::domain::user_data::SaveUserSettings;
use crate::userdata::service::UserSettingsService;
use actix_web::dev::HttpServiceFactory;
use actix_web::middleware::from_fn;
use actix_web::{web, HttpResponse, Responder};
use std::sync::Arc;
use sqlx::PgPool;
use tracing::{debug, error};
use crate::userdata::repositories::diets_repo::{DietRepoBuilder, DietRepository};

pub fn user_settings_routes(firebase: Arc<Firebase>, db_pool:Arc<PgPool>) -> impl HttpServiceFactory {
    let diet_repo: Arc<dyn DietRepository> = Arc::new(DietRepoBuilder::postgres(db_pool.clone()));
    let user_settings_service = Arc::new(UserSettingsService::new(diet_repo));

    web::scope("/v1/settings")
        .app_data(web::Data::new(user_settings_service.clone()))
        .wrap(from_fn(authorize(firebase.clone())))
        .route("", web::get().to(save_user_data))
        .route("/diets", web::get().to(get_diets))
}

// async fn get_user_data(user_id: web::Path<String>) -> impl Responder {
//     println!("user_id: {}", user_id);
//     HttpResponse::Ok()
// }

async fn save_user_data(user_data: web::Json<UserData>, user_settings: web::Data<Arc<UserSettingsService>>) -> impl Responder {
    let user_data: SaveUserSettings = user_data.0.into();
    match user_settings.get_ref().save(user_data).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            error!("Error saving user_data: {}", e);
            HttpResponse::InternalServerError().body("Unexpected error")
        }
    }
}

async fn get_diets(user_settings: web::Data<Arc<UserSettingsService>>) -> impl Responder {
    debug!("Calling get diets");
    let diets = user_settings.get_diets().await;
    match diets {
        Ok(diets) => HttpResponse::Ok().json(diets),
        Err(err) => {
            error!("Error getting diets: {}", err);
            err.to_http_error()
        }
    }

}
