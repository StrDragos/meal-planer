use crate::common::firebase::Firebase;
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::middleware::Next;
use futures_util::future::LocalBoxFuture;
use futures_util::FutureExt;
use std::sync::Arc;
use actix_web::HttpMessage;
use tracing::{debug, warn};

pub fn authorize<B>(firebase: Arc<Firebase>) -> impl Fn(ServiceRequest, Next<B>) -> LocalBoxFuture<'static, Result<ServiceResponse<B>, actix_web::Error>>
where
    B: MessageBody + 'static,
{
    move |request, next| {
        let fb = firebase.clone();
        async move {

            let header_value = request.headers().get("Authorization").ok_or_else(||{
                debug!("Authorization header missing");
                actix_web::error::ErrorUnauthorized("Unauthorized")
            })?;

            let header_str = header_value.to_str().map_err(|_| {
                debug!("Invalid header encoding");
                actix_web::error::ErrorUnauthorized("Invalid header encoding")
            })?;

            let token = header_str.strip_prefix("Bearer ").ok_or_else(|| {
                debug!("Invalid Bearer Token format");
                actix_web::error::ErrorUnauthorized("Invalid Bearer Token")
            })?;

            let user_data = fb.get_user_data(token).await.map_err(|err| {
                warn!("Error getting user data: {}", err);
                actix_web::error::ErrorUnauthorized("Unauthorized")
            })?;

            request.extensions_mut().insert(user_data);
            next.call(request).await
        }.boxed_local()
    }
}



