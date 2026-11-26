use axum::http::StatusCode;

use bajam_common::models::user::User;
use sqlx::SqlitePool;
use crate::state::AppState;

pub async fn sql_username_from_id(state: &AppState, user_id: &i64) -> Result<String, (StatusCode, String)> {
  let result = sqlx::query_scalar(
    r#"
      SELECT username FROM users
      WHERE id = (?)
    "#
  )
  .bind(user_id)
  .fetch_one(&state.db)
  .await
  .map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error : {e}"))
    })?;

    Ok(result)
}

pub async fn sql_find_user_by_username(db: &SqlitePool, username: &str) -> sqlx::Result<Option<User>> {
  sqlx::query_as!(User, "SELECT * FROM users WHERE username = $1", username)
    .fetch_optional(db)
    .await
}

pub async fn sql_insert_user(db: &SqlitePool, username: &str, password_hash: &str) -> sqlx::Result<()> {
  sqlx::query("INSERT into users (username, password_hash) VALUES (?, ?)")
    .bind(username)
    .bind(password_hash)
    .execute(db)
    .await?;
  Ok(())
}
