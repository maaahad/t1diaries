use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    // pub acquire_timeout: std::time::Duration,
    // pub idle_timeout: std::time::Duration,
    // pub max_lifetime: std::time::Duration,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn database_config_accepts_valid_values() {
        let config = DatabaseConfig {
            url: String::from("postgres://user:password@localhost:5432/t1diaries"),
            max_connections: 10,
            min_connections: 5,
        };
        assert_eq!(config.max_connections, 10);
        assert_eq!(config.min_connections, 5);
    }
}
