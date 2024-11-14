// use std::task::{Context, Poll};
// use actix_service::{Service, Transform};
// use actix_web::dev::{ServiceRequest, ServiceResponse};
// use anyhow::Error;
// use serde::{Deserialize, Serialize};
//
// #[derive(Deserialize, Debug, Serialize)]
// struct Claims {
//     sub: String,
//     email: String,
// }
//
// //1. get the header of the request
// //2. get the jwt token and verify it with google
// //3. extract claims and propagate the values through middleware add them into storage
//
// struct Authorization;
//
// impl <S, B> Transform<S, ServiceRequest> for Authorization
// where
//     S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
//     S::Future: 'static,
//     B: 'static,
// {
//     type Response = ServiceResponse<B>;
//     type Error = Error;
//     type Transform = ();
//     type InitError = ();
//     type Future = ();
//
//     fn new_transform(&self, service: S) -> Self::Future {
//         todo!()
//     }
// }
//
// struct AuthorizationMiddleware <S> {
//     service: S
// }
//
// impl <S, B> Service<ServiceRequest> for AuthorizationMiddleware<S>
// where
//     S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
//     S::Future: 'static,
//     B: 'static,
// {
//     type Response = ServiceResponse<B>;
//     type Error = Error;
//     type Future = LocalBoxFuture;
//
//     fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
//         todo!()
//     }
//
//     fn call(&self, req: ServiceRequest) -> Self::Future {
//         todo!()
//     }
// }
//
