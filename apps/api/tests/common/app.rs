use api::router::build_router;
use axum::Router;
use database::Database;

use crate::common::{config::TestAppConfig, state::TestAppState};

pub struct TestApp {
    pub app: Router,
}

impl TestApp {
    pub async fn new() -> Self {
        let config = TestAppConfig::new().build();

        let schema = graphql::build_schema();

        let database = Database::connect(&config.database).await.unwrap();

        let state = TestAppState::new(config, schema, database).build();

        Self {
            app: build_router(state.clone()),
        }
    }

    pub fn build(self) -> Router {
        self.app
    }
}
