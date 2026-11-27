use axum::{Extension, extract::{Path, State}, http::StatusCode};
use crate::{auth::middleware::AuthContext, state::AppState};
use super::service::delete_escapade;
use crate::escapades::error::to_http;


pub async fn escapade_delete(
    State(state): State<AppState>,
    Path(escapade_id): Path<i64>,
    Extension(auth_context): Extension<AuthContext>, // User id obtenu avec le auth middleware
) -> Result<(), (StatusCode, String)> {

    delete_escapade(&state.db, auth_context.user_id, escapade_id).await.map_err(to_http)?;

    Ok(())
}
