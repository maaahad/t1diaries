use database::{Database, DatabaseConfig};
use std::time::Duration;

#[tokio::test]
async fn connects_to_postgresql() {
    let config = DatabaseConfig::new(
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
        10,
        2,
        Duration::from_secs(5),
        Duration::from_secs(600),
        Duration::from_secs(1800),
    )
    .expect("valid database configuratin");

    let database = Database::connect(&config)
        .await
        .expect("database should connect");

    database
        .health_check()
        .await
        .expect("database health check should succeed")
}
