use api::router::build_router;
use axum::Router;

use crate::common::{config::TestAppConfig, state::TestAppState};

pub struct TestApp {
    pub app: Router,
}

impl TestApp {
    pub fn new() -> Self {
        let config = TestAppConfig::new().build();

        let schema = graphql::build_schema();

        let state = TestAppState::new(config, schema).build();

        Self {
            app: build_router(state.clone()),
        }
    }

    pub fn build(self) -> Router {
        self.app
    }
}
