use axum::{Extension, extract::State, http::StatusCode};
use bajam_common::models::escapade::Escapade;
use crate::auth::middleware::AuthContext;
use crate::escapades::error::to_http;
use crate::state::AppState;
use axum::Json;
use crate::escapades::list::service::list_escapades;

pub async fn escapades_list_api(
    State(state): State<AppState>,
    Extension(auth_context): Extension<AuthContext>, // User id obtenu avec le auth middleware
) -> Result<Json<Vec<Escapade>>, (StatusCode, String)> {

  let list = list_escapades(&state.db, auth_context.user_id).await.map_err(to_http)?;

  Ok(Json(list))
}
