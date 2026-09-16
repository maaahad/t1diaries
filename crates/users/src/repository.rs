use crate::{User, error::UserRepositoryError};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(user: User) -> Result<(), UserRepositoryError> {
        todo!()
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, UserRepositoryError> {
        todo!()
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>, UserRepositoryError> {
        todo!()
    }
}
