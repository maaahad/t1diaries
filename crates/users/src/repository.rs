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

    pub async fn create(&self, user: &User) -> Result<(), UserRepositoryError> {
        sqlx::query!(
            r#"
            INSERT INTO users (
                id, 
                email, 
                created_at, 
                updated_at
            )
            VALUES ($1, $2, $3, $4)
            "#,
            user.id,
            user.email,
            user.created_at,
            user.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(UserRepositoryError::Create)?;

        Ok(())
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, UserRepositoryError> {
        sqlx::query_as!(
            User,
            r#"
                SELECT
                    id,
                    email,
                    created_at,
                    updated_at
                FROM users
                WHERE id = $1
                "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(UserRepositoryError::Create)
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>, UserRepositoryError> {
        sqlx::query_as!(
            User,
            r#"
                SELECT
                    id,
                    email,
                    created_at,
                    updated_at
                FROM users
                WHERE LOWER(email) = LOWER($1)
                "#,
            email
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(UserRepositoryError::Find)
    }
}
