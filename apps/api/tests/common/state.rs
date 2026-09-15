use api::state::AppState;
use app_config::AppConfig;
use database::Database;
use graphql::AppSchema;

pub struct TestAppState {
    state: AppState,
}

impl TestAppState {
    pub fn new(config: AppConfig, schema: AppSchema, database: Database) -> Self {
        TestAppState {
            state: AppState::new(config, schema, database),
        }
    }

    pub fn build(self) -> AppState {
        self.state
    }
}
