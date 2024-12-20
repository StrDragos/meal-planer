use std::sync::Arc;
use actix_web::dev::HttpServiceFactory;
use actix_web::{web, HttpResponse};
use actix_web::middleware::from_fn;
use crate::common::firebase::Firebase;
use crate::middlwares::authorization::authorize;

pub fn dish_routes(firebase: Arc<Firebase>) -> impl HttpServiceFactory  {
   web::scope("/v1/dishes")
       .wrap(from_fn(authorize(firebase)))
       .route("", web::get().to(list_dishes))
}

async fn list_dishes() -> HttpResponse {
    HttpResponse::Ok().finish()
}