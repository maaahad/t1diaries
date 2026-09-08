use serde::Deserialize;

use crate::error::DatabaseError;

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    // pub acquire_timeout: std::time::Duration,
    // pub idle_timeout: std::time::Duration,
    // pub max_lifetime: std::time::Duration,
}

impl DatabaseConfig {
    pub fn new(
        url: String,
        max_connections: u32,
        min_connections: u32,
    ) -> Result<Self, DatabaseError> {
        Ok(DatabaseConfig {
            url,
            max_connections,
            min_connections,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn database_config_accepts_valid_values() {
        let config = DatabaseConfig::new(
            String::from("postgres://user:password@localhost:5432/t1diaries"),
            10,
            5,
        );

        assert!(config.is_ok());

        let DatabaseConfig {
            max_connections,
            min_connections,
            ..
        } = config.unwrap();

        assert_eq!(max_connections, 10);
        assert_eq!(min_connections, 5);
    }
}
