use axum::{Extension, extract::State, http::StatusCode};
use bajam_common::models::escapade::Escapade;
use crate::state::AppState;
use crate::db::escapade_queries::sql_list_escapades;
use axum::Json;


pub async fn escapades_list(
    State(state): State<AppState>,
    Extension(user_id): Extension<i64>, // User id obtenu avec le auth middleware
) -> Result<Json<Vec<Escapade>>, (StatusCode, String)> {

  print!("Escapade list request received... ");

  let list = sql_list_escapades(&state, &user_id).await?;

  Ok(Json(list))
}
