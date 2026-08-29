use app_config::Config;
use graphql::AppSchema;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub schema: AppSchema,
}

impl AppState {
    pub fn new(config: Config, schema: AppSchema) -> Self {
        Self {
            config: Arc::new(config),
            schema,
        }
    }

    #[cfg(test)]
    pub fn test() -> Self {
        todo!("test state will be implemented")
    }
}
