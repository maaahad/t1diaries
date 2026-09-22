use chrono::Utc;
use sqlx::PgPool;
use users::{User, UserRepository};
use uuid::Uuid;

#[sqlx::test(migrations = "../../migrations")]
pub async fn create_persists_user(pool: sqlx::PgPool) {
    let repository = UserRepository::new(pool.clone());
    let now = Utc::now();
    let user = User {
        id: Uuid::new_v4(),
        email: String::from("example@example.com"),
        created_at: now,
        updated_at: Some(now),
    };

    repository
        .create(&user)
        .await
        .expect("user should be created");

    let row = sqlx::query!(
        r#"
        SELECT id, email, created_at, updated_at
        FROM users
        WHERE id = $1
        "#,
        user.id
    )
    .fetch_one(&pool)
    .await
    .expect("user should exist");

    assert_eq!(user.id, row.id);
    assert_eq!(user.email, row.email);
    assert_eq!(user.created_at, row.created_at);
    assert_eq!(user.updated_at, row.updated_at);
}

#[sqlx::test(migrations = "../../migrations")]
pub async fn find_by_id_returns_existing_user(pool: PgPool) {
    let repository = UserRepository::new(pool.clone());
    let now = Utc::now();
    let user = User {
        id: Uuid::new_v4(),
        email: "example@example.com".into(),
        created_at: now,
        updated_at: Some(now),
    };

    repository
        .create(&user)
        .await
        .expect("user should be created");

    let result = repository
        .find_by_id(user.id)
        .await
        .expect("query should succeed");

    assert_eq!(result, Some(user));
}

#[sqlx::test(migrations = "../../migrations")]
pub async fn find_by_id_returns_none_for_missing_user(pool: PgPool) {
    let repository = UserRepository::new(pool.clone());
    let result = repository
        .find_by_id(Uuid::new_v4())
        .await
        .expect("query should succeed");

    assert_eq!(result, None);
}

#[sqlx::test(migrations = "../../migrations")]
pub async fn find_by_email_returns_existing_user(pool: PgPool) {
    let repository = UserRepository::new(pool.clone());
    let now = Utc::now();
    let email = "example@example.com";

    let user = User {
        id: Uuid::new_v4(),
        email: email.to_owned(),
        created_at: now,
        updated_at: Some(now),
    };

    repository
        .create(&user)
        .await
        .expect("user should be created");

    let result = repository
        .find_by_email(email)
        .await
        .expect("query should succeed");

    assert_eq!(result, Some(user));
}

#[sqlx::test(migrations = "../../migrations")]
pub async fn find_by_email_is_case_insensitive(pool: PgPool) {
    let repository = UserRepository::new(pool.clone());
    let now = Utc::now();

    let user = User {
        id: Uuid::new_v4(),
        email: "example@example.com".to_owned(),
        created_at: now,
        updated_at: Some(now),
    };

    repository
        .create(&user)
        .await
        .expect("user should be created");

    let result = repository
        .find_by_email("EXAMPLE@EXAMPLE.COM")
        .await
        .expect("query should succeed");

    assert_eq!(result, Some(user));
}
