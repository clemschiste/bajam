use axum::http::StatusCode;
use bajam_common::models::escapade::Escapade;

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

/// List escapades the user is a member of
pub async fn sql_list_escapades(state: &AppState, user_id: &i64) -> Result<Vec<Escapade>, (StatusCode, String)> {

  let result = sqlx::query_as!(
    Escapade,
    r#"
        SELECT name, id
        FROM escapades
        INNER JOIN escapades_map
          ON escapades.id = escapades_map.escapade_id
        WHERE escapades_map.user_id = ?
        LIMIT 100
    "#,
    user_id
  )
  .fetch_all(&state.db)
    .await
    .map_err(|e| {
        eprintln!("Database error {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error : {e}"))
    })?;

    Ok(result)

}
