use futures_util::future::err;
use sqlx::Error;
use thiserror::Error;
use tracing::error;

#[derive(Debug, Error)]
pub enum DbError {
    #[error("Connection error")]
    ConnectionError,
    #[error("Unexpected error: {0}")]
    Unexpected(String),
    #[error("Not found")]
    NotFound
}

impl From<sqlx::Error> for DbError {
    fn from(value: Error) -> Self {
        error!("Error: {:?}",value);
        match value {
            Error::PoolTimedOut => DbError::ConnectionError,
            Error::RowNotFound => DbError::NotFound,
            Error::Database(err) => DbError::Unexpected(err.to_string()),
            _ => DbError::Unexpected(value.to_string())
        }
    }
}