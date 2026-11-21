use axum::extract::Path;
use axum::{Extension, extract::State, http::StatusCode};
use bajam_common::models::heartbeat::Heartbeat;
use crate::db::heartbeat_queries::sql_list_heartbeats;
use crate::state::AppState;
use axum::Json;


pub async fn heartbeats_list(
    State(state): State<AppState>,
    Extension(user_id): Extension<i64>,
    Path(escapade_id): Path<i64>
) -> Result<Json<Vec<Heartbeat>>, (StatusCode, String)> {

  let list = sql_list_heartbeats(&state, &user_id, &escapade_id).await?;

  Ok(Json(list))
}
