// use async_graphql::{Error as GqlError, ErrorExtensions};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("not found")]
    NotFound,

    #[error("validation failed: {0}")]
    Validation(String),

    #[error("unauthenticated")]
    Unauthenticated,

    #[error("forbidden")]
    Forbidden,

    #[error("conflict: {0}")]
    Conflict(String),

    #[error("internal error")]
    Internal(#[from] anyhow::Error),

    #[error("database error")]
    Database(#[from] sqlx::Error),
}
