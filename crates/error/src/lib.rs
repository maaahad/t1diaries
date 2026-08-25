use async_graphql::{Error as GqlError, ErrorExtensions};
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

// Single place that map our domain errors into GraphQl Errors with
// extensions.code that Frontend can branch on (ex. UNAUTHENTICATED)
impl ErrorExtensions for AppError {
    fn extend(&self) -> GqlError {
        GqlError::new(self.to_string()).extend_with(|_, e| {
            let code = match self {
                AppError::NotFound => "NOT_FOUND",
                AppError::Validation(_) => "VALIDATION",
                AppError::Unauthenticated => "UNAUTHENTICATED",
                AppError::Forbidden => "FORBIDDEN",
                AppError::Conflict(_) => "CONFLICT",
                AppError::Internal(_) | AppError::Database(_) => "INTERNAL",
            };
            e.set("code", code);
        })
    }
}
