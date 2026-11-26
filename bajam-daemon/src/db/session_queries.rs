use bajam_common::models::session::Session;
use chrono::NaiveDateTime;
use sqlx::SqlitePool;
use crate::state::AppState;

pub async fn sql_delete_expired_sessions(state: &AppState) -> anyhow::Result<u64> {
  let result = sqlx::query("DELETE FROM sessions WHERE expired_at < datetime('now')")
    .execute(&state.db)
    .await?;

  // return number of sessions deleted
  Ok(result.rows_affected())
}

pub async fn sql_insert_session(db: &SqlitePool, token_hash: &str, user_id: i64, expires_at: NaiveDateTime) -> sqlx::Result<()> {
  sqlx::query("INSERT INTO sessions (token_hash, user_id, expires_at) VALUES (?, ?, ?)")
    .bind(token_hash)
    .bind(user_id)
    .bind(expires_at)
    .execute(db)
    .await?;
  Ok(())
}

pub async fn sql_find_session_by_hash(db: &SqlitePool, hash: &str) -> sqlx::Result<Option<Session>> {
  sqlx::query_as!(
    Session,
    r#"
          SELECT id, user_id, token_hash, expires_at
          FROM sessions
          WHERE token_hash = $1
        "#,
    hash
  )
  .fetch_optional(db)
  .await
}

pub async fn sql_delete_user_session(db: &SqlitePool, user_id: i64, session_id: i64) -> sqlx::Result<()> {
  sqlx::query("DELETE FROM sessions WHERE user_id = (?) AND id = (?)")
    .bind(user_id)
    .bind(session_id)
    .execute(db)
    .await?;
  Ok(())
}

pub async fn sql_delete_all_user_sessions(db: &SqlitePool, user_id: i64) -> sqlx::Result<()> {
  sqlx::query("DELETE FROM sessions WHERE user_id = (?)")
    .bind(user_id)
    .execute(db)
    .await?;
  Ok(())
}
