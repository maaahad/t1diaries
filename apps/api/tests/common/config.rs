use app_config::{AppConfig, Environment, GraphqlConfig, MetaConfig, ServerConfig};

pub struct TestAppConfig {
    config: AppConfig,
}

impl TestAppConfig {
    pub fn new() -> Self {
        TestAppConfig {
            config: AppConfig {
                meta: MetaConfig {
                    name: "t1diaries".into(),
                    environment: Environment::Test,
                },
                server: ServerConfig {
                    host: "127.0.0.1".into(),
                    port: 0,
                },
                graphql: GraphqlConfig { graphiql: false },
            },
        }
    }

    pub fn build(self) -> AppConfig {
        self.config
    }
}
