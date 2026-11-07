use axum::{Extension, extract::{State, Path}, http::StatusCode};
use crate::state::AppState;
use crate::db::escapade_queries::sql_delete_escapade;


pub async fn escapade_delete(
    State(state): State<AppState>,
    Path(escapade_id): Path<i64>,
    Extension(user_id): Extension<i64>, // User id obtenu avec le auth middleware
) -> Result<StatusCode, (StatusCode, String)> {

  print!("Escapade deletion request received... ");

  sql_delete_escapade(&state, &user_id, &escapade_id).await?;

  println!("Done. Escapade deleted.");

  Ok(StatusCode::NO_CONTENT)  
}
