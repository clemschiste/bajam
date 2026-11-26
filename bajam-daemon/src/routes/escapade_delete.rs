use axum::{Extension, extract::{State, Path}, http::StatusCode};
use crate::{auth::middleware::AuthContext, state::AppState};
use crate::db::escapade_queries::sql_delete_escapade;

pub async fn escapade_delete(
    State(state): State<AppState>,
    Path(escapade_id): Path<i64>,
    Extension(auth_context): Extension<AuthContext>, // User id obtenu avec le auth middleware
) -> Result<StatusCode, (StatusCode, String)> {

  print!("Escapade deletion request received... ");

  sql_delete_escapade(&state, &auth_context.user_id, &escapade_id).await?;

  println!("Done. Escapade deleted.");

  Ok(StatusCode::NO_CONTENT)  
}
