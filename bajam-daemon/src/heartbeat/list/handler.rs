use axum::extract::Path;
use axum::{Extension, extract::State, http::StatusCode};
use bajam_common::models::heartbeat::Heartbeat;
use crate::auth::middleware::AuthContext;
use crate::heartbeat::error::to_http;
use crate::state::AppState;
use axum::Json;
use crate::heartbeat::list::service::list_heartbeats;

pub async fn heartbeat_list_api(
    State(state): State<AppState>,
    Extension(auth_context): Extension<AuthContext>,
    Path(escapade_id): Path<i64>
) -> Result<Json<Vec<Heartbeat>>, (StatusCode, String)> {

  let list = list_heartbeats(&state.db, auth_context.user_id, escapade_id).await.map_err(to_http)?;

  Ok(Json(list))
}
