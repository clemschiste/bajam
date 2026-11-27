use axum::{Extension, extract::State, http::StatusCode};
use bajam_common::models::escapade::Escapade;
use crate::auth::middleware::AuthContext;
use crate::state::AppState;
use crate::db::escapade_queries::sql_list_escapades;
use axum::Json;


pub async fn escapades_list(
    State(state): State<AppState>,
    Extension(auth_context): Extension<AuthContext>, // User id obtenu avec le auth middleware
) -> Result<Json<Vec<Escapade>>, (StatusCode, String)> {

  print!("Escapade list request received... ");

  let list = sql_list_escapades(&state, &auth_context.user_id).await?;

  Ok(Json(list))
}
