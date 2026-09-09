use app_config::{AppConfig, Environment, GraphqlConfig, MetaConfig, ServerConfig};
use database::config::DatabaseConfig;
use std::time::Duration;

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
                database: DatabaseConfig {
                    url: String::from("postgres://user:password@localhost:5432/t1diaries"),
                    max_connections: 10,
                    min_connections: 5,
                    acquire_timeout: Duration::from_secs(5),
                    idle_timeout: Duration::from_secs(600),
                    max_lifetime: Duration::from_secs(1800),
                },
            },
        }
    }

    pub fn build(self) -> AppConfig {
        self.config
    }
}
