use thiserror::Error;

/**
 Error map :
 PostgreSQL -> sqlx::Error -> DatabaseError -> AppError -> GraphQL Error
*/

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("invalid database configuration: {0}")]
    InvalidConfiguration(&'static str),

    #[error("failed to connect to PostgreSQL")]
    ConnectionFailed(#[source] sqlx::Error),

    #[error("database health check failed")]
    HealthCheckFailed(#[source] sqlx::Error),

    #[error("database migration failed")]
    Migration(#[source] sqlx::migrate::MigrateError),
}
