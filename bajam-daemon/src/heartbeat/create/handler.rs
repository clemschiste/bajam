use axum::extract::Path;
use axum::{Extension, extract::State, http::StatusCode};
use bajam_common::models::heartbeat::HeartbeatPost;
use crate::auth::middleware::AuthContext;
use crate::heartbeat::error::to_http;
use crate::state::AppState;
use axum::{Form, Json};
use crate::heartbeat::create::service::create_heartbeat;

pub async fn heartbeat_post_api(
    State(state): State<AppState>,
    Extension(auth_context): Extension<AuthContext>,
    Path(escapade_id): Path<i64>,
    Json(post): Json<HeartbeatPost>,
    
) -> Result<(), (StatusCode, String)> {

  create_heartbeat(&state.db, auth_context.user_id, escapade_id, post).await.map_err(to_http)?;

  Ok(())

}

pub async fn heartbeat_post_web(
    State(state): State<AppState>,
    Extension(auth_context): Extension<AuthContext>,
    Path(escapade_id): Path<i64>,
    Form(post): Form<HeartbeatPost>,
    
) -> Result<(), (StatusCode, String)> {

  create_heartbeat(&state.db, auth_context.user_id, escapade_id, post).await.map_err(to_http)?;

  Ok(())

}
