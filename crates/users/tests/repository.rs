use chrono::Utc;
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
        updated_at: now,
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
    assert_eq!(
        user.updated_at,
        row.updated_at.expect("created_at should be exist")
    );
}
