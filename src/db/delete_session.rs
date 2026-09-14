use axum::http::StatusCode;
use crate::state::AppState;

pub async fn sql_delete_session(state: AppState, user_id: i64) -> Result<(), (StatusCode, String)> {
  sqlx::query("DELETE FROM sessions WHERE user_id = (?)")
    .bind(user_id)
    .execute(&state.db)
    .await
    .map_err(|e| {
        eprintln!("Database error {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error : {e}"))
    })?;

  Ok(())
  
}
