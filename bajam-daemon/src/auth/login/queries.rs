use bajam_common::models::user::User;
use sqlx::SqlitePool;

pub async fn sql_find_user_by_username(db: &SqlitePool, username: &str) -> sqlx::Result<Option<User>> {
  sqlx::query_as!(User, "SELECT * FROM users WHERE username = $1", username)
    .fetch_optional(db)
    .await
}
