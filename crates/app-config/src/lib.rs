use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

impl AppConfig {
    pub fn load() -> Result<Self, config::ConfigError> {
        let builder =
            config::Config::builder().add_source(config::File::with_name("config/default"));
        // TODO:  add_source for .env file (with secret for local development)

        builder.build()?.try_deserialize()
    }
}
