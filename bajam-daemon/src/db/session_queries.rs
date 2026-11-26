use crate::state::AppState;

pub async fn sql_delete_expired_sessions(state: &AppState) -> anyhow::Result<u64> {
  let result = sqlx::query("DELETE FROM sessions WHERE expired_at < datetime('now')")
    .execute(&state.db)
    .await?;

  // return number of sessions deleted
  Ok(result.rows_affected())
}
