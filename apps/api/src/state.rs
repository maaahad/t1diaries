use app_config::AppConfig;
use graphql::AppSchema;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub schema: AppSchema,
}

impl AppState {
    pub fn new(config: AppConfig, schema: AppSchema) -> Self {
        Self {
            config: Arc::new(config),
            schema,
        }
    }
}
