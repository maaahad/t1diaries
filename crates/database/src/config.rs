use crate::error::DatabaseError;
use serde::Deserialize;
use std::time::Duration;

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub acquire_timeout: Duration,
    pub idle_timeout: Duration,
    pub max_lifetime: Duration,
}

impl DatabaseConfig {
    pub fn new(
        url: String,
        max_connections: u32,
        min_connections: u32,
        acquire_timeout: Duration,
        idle_timeout: Duration,
        max_lifetime: Duration,
    ) -> Result<Self, DatabaseError> {
        if max_connections == 0 {
            return Err(DatabaseError::InvalidConfiguration(
                "max_connection should be greater than zero",
            ));
        }

        if max_connections < min_connections {
            return Err(DatabaseError::InvalidConfiguration(
                "max_connections should be greater than min_connections",
            ));
        }

        Ok(DatabaseConfig {
            url,
            max_connections,
            min_connections,
            acquire_timeout,
            idle_timeout,
            max_lifetime,
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
            Duration::from_secs(5),
            Duration::from_secs(600),
            Duration::from_secs(1800),
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

    #[test]
    fn rejects_zero_max_connections() {
        let config = DatabaseConfig::new(
            String::from("postgres://user:password@localhost:5432/t1diaries"),
            0,
            5,
            Duration::from_secs(5),
            Duration::from_secs(600),
            Duration::from_secs(1800),
        );

        assert!(matches!(
            config,
            Err(DatabaseError::InvalidConfiguration(
                "max_connection should be greater than zero"
            ))
        ));
    }

    #[test]
    fn rejects_min_connections_greater_than_max_connections() {
        let config = DatabaseConfig::new(
            String::from("postgres://user:password@localhost:5432/t1diaries"),
            4,
            5,
            Duration::from_secs(5),
            Duration::from_secs(600),
            Duration::from_secs(1800),
        );

        assert!(matches!(
            config,
            Err(DatabaseError::InvalidConfiguration(
                "max_connections should be greater than min_connections"
            ))
        ));
    }
}
