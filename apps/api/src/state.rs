use app_config::AppConfig;
use database::Database;
use graphql::AppSchema;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub schema: AppSchema,
    pub database: Database,
}

impl AppState {
    pub fn new(config: AppConfig, schema: AppSchema, database: Database) -> Self {
        Self {
            config: Arc::new(config),
            schema,
            database,
        }
    }
}
