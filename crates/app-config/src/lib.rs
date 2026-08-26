use config::ConfigError;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub app: AppConig,
    pub server: ServerConfig,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Environment {
    Local,
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

impl Config {
    pub fn load() -> Result<Self, ConfigError> {
        let builder =
            config::Config::builder().add_source(config::File::with_name("config/default"));
        // TODO:  add_source for .env file (with secret for local development)

        builder.build()?.try_deserialize()
    }

    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.server.port == 0 {
            // FIXME: need right error type
            return Err(ConfigError::Message("server port can not be 0".into()));
        }

        Ok(())
    }
}
