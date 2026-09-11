use sqlx::{PgPool, postgres::PgPoolOptions};

use crate::config::DatabaseConfig;
use crate::error::DatabaseError;
use crate::migration;

#[derive(Clone)]
pub struct Database {
    pool: PgPool,
}

impl Database {
    pub async fn connect(config: &DatabaseConfig) -> Result<Self, DatabaseError> {
        let pool = PgPoolOptions::new()
            .max_connections(config.max_connections)
            .min_connections(config.min_connections)
            .connect(&config.url)
            .await
            .map_err(DatabaseError::ConnectionFailed)?;

        Ok(Self { pool })
    }

    pub async fn migrate(&self) -> Result<(), DatabaseError> {
        migration::run(&self.pool)
            .await
            .map_err(DatabaseError::Migration)
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }
}
