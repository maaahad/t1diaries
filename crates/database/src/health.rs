use sqlx::Executor;

use crate::{error::DatabaseError, pool::Database};

impl Database {
    pub async fn health_check(&self) -> Result<(), DatabaseError> {
        sqlx::query("SELECT 1")
            .execute(self.pool())
            .await
            .map(|_| ())
            .map_err(DatabaseError::HealthCheckFailed)
    }
}
