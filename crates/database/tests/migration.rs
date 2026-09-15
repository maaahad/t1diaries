use database::{Database, DatabaseConfig};
use std::time::Duration;

#[tokio::test]
async fn applies_pending_migration() {
    // TODO: (maaahad) this is repeated here, in connection and some other places.
    //     Lets expose a TestDatabase::build instead
    let config = DatabaseConfig::new(
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
        10,
        2,
        Duration::from_secs(5),
        Duration::from_secs(600),
        Duration::from_secs(1800),
    )
    .expect("valid database configuration");

    let database = Database::connect(&config)
        .await
        .expect("database should connect");

    database
        .migrate()
        .await
        .expect("first migration should succeed");

    database
        .migrate()
        .await
        .expect("second migration should succeed");

    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) from _sqlx_migrations")
        .fetch_one(database.pool())
        .await
        .expect("migration history should exist");

    assert_eq!(count, 1);
}
