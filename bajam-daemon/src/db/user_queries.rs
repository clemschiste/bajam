use axum::http::StatusCode;

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
