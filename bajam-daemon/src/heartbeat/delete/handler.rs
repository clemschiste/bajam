use axum::{Extension, extract::{State, Path}, http::StatusCode};
use crate::{auth::middleware::AuthContext, heartbeat::error::to_http, state::AppState};
use super::service::delete_heartbeat;


pub async fn heartbeat_delete(
    State(state): State<AppState>,
    Path(heartbeat_id): Path<i64>,
    Extension(auth_context): Extension<AuthContext>, // User id obtenu avec le auth middleware
) -> Result<(), (StatusCode, String)> {

  delete_heartbeat(&state.db, auth_context.user_id, heartbeat_id).await.map_err(to_http)?;

  Ok(())

}
