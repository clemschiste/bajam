use axum::http::StatusCode;
use bajam_common::models::heartbeat::Heartbeat;

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

/// List heartbeat of an escapade.
pub async fn sql_list_heartbeats(
  state: &AppState,
  user_id: &i64,
  escapade_id: &i64,
) -> Result<Vec<Heartbeat>, (StatusCode, String)> {

  let result = sqlx::query_as!(
    Heartbeat,
    r#"
        SELECT *
        FROM heartbeats
        WHERE
          escapade_id = ?
          AND user_id = ?
        LIMIT 500
    "#,
    escapade_id,
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
