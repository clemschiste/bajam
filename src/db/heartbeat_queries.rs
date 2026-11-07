use axum::http::StatusCode;

use crate::state::AppState;

/// Deletes the selected heartbeat own by a user.
pub async fn sql_delete_heartbeat(state: &AppState, user_id: &i64, heartbeat_id: &i64) -> Result<(), (StatusCode, String)> {
  sqlx::query(
    r#"
      DELETE FROM heartbeats WHERE id = (?) AND user_id = (?)
    "#
  )
  .bind(heartbeat_id)
  .bind(user_id)
  .execute(&state.db)
    .await
    .map_err(|e| {
        eprintln!("Database error {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error : {e}"))
    })?;

  Ok(())
}
