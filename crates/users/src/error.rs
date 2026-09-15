use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserRepositoryError {
    #[error("failed to create user")]
    Create(#[source] sqlx::Error),

    #[error("failed to find user")]
    Find(#[source] sqlx::Error),

    #[error("failed to update user")]
    Update(#[source] sqlx::Error),

    #[error("failed to delete user")]
    Delete(#[source] sqlx::Error),
}
