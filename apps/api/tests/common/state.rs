use api::state::AppState;
use app_config::AppConfig;
use graphql::AppSchema;

pub struct TestAppState {
    state: AppState,
}

impl TestAppState {
    pub fn new(config: AppConfig, schema: AppSchema) -> Self {
        TestAppState {
            state: AppState::new(config, schema),
        }
    }

    pub fn build(self) -> AppState {
        self.state
    }
}
