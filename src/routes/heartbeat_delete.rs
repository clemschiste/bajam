use axum::{Extension, extract::{State, Path}, http::StatusCode};
use crate::state::AppState;
use crate::db::heartbeat_queries::sql_delete_heartbeat;


pub async fn heartbeat_delete(
    State(state): State<AppState>,
    Path(heartbeat_id): Path<i64>,
    Extension(user_id): Extension<i64>, // User id obtenu avec le auth middleware
) -> Result<StatusCode, (StatusCode, String)> {

  print!("Heartbeat deletion request received... ");

  sql_delete_heartbeat(&state, &user_id, &heartbeat_id).await?;

  println!("Done. Heartbeat deleted.");
      
  Ok(StatusCode::NO_CONTENT)  
}
