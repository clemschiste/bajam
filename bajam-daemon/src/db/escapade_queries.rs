use axum::http::StatusCode;

use crate::state::AppState;

/// Deletes the selected escapade belonging to the user (must be admin).
pub async fn sql_delete_escapade(state: &AppState, user_id: &i64, escapade_id: &i64) -> Result<(), (StatusCode, String)> {

  let result = sqlx::query(
    r#"
      DELETE FROM escapades
      WHERE id = (?)
      AND EXISTS (
        SELECT 1
        FROM escapades_map
        WHERE escapade_id = ?
        AND user_id = ?
        AND role = 'admin'
      )
    "#
  )
  .bind(escapade_id)
  .bind(escapade_id)
  .bind(user_id)
  .execute(&state.db)
    .await
    .map_err(|e| {
        eprintln!("Database error {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error : {e}"))
    })?;

    if result.rows_affected() == 0 {
      return Err((StatusCode::NOT_FOUND, "Escapade not found or insufficient permissions".to_string()));
    }

  Ok(())
}
