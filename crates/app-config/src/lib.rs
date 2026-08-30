use config::{ConfigError, File};
use dotenvy::dotenv;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub app: AppConig,
    pub server: ServerConfig,
    pub graphql: GraphqlConfig,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Environment {
    Development,
    Test,
    Production,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AppConig {
    pub name: String,
    pub environment: Environment,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GraphqlConfig {
    pub graphiql: bool,
}

// TODO: (maaahad) let's rename it to AppConfig instead
impl Config {
    pub fn load() -> Result<Self, ConfigError> {
        let _ = dotenv();
        let environment = std::env::var("APP_ENV").unwrap_or_else(|_| "local".to_owned());

        let builder = config::Config::builder()
            .add_source(File::with_name("config/base"))
            .add_source(File::with_name(&format!("config/{environment}")).required(false))
            .add_source(
                config::Environment::default()
                    .separator("__")
                    .try_parsing(true),
            );

        let config: Config = builder.build()?.try_deserialize()?;

        config.validate()?;

        Ok(config)
    }

    // TODO: (maaahad) this is not done yet, just testing
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.server.port == 0 {
            return Err(ConfigError::Message("server port can not be 0".into()));
        }

        Ok(())
    }
}
