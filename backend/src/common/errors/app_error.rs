use crate::common::errors::db_error::DbError;
use actix_web::HttpResponse;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Entity {0} not found")]
    NotFound(String),
    #[error("Internal error: {0}")]
    InternalError(String),
}

impl AppError {
    pub fn from_db_error(err: DbError) -> Self {
        match err {
            DbError::NotFound => Self::NotFound(err.to_string()),
            err => Self::InternalError(err.to_string()),
        }
    }

    pub fn to_http_error(&self) -> HttpResponse {
        match self {
            Self::NotFound(_) => HttpResponse::NotFound().finish(),
            Self::InternalError(_) => HttpResponse::InternalServerError().finish(),
        }
    }
}
