use axum::{Extension, extract::State, http::StatusCode};
use crate::state::AppState;
use crate::db::delete_session::sql_delete_session;


pub async fn session_logout(
    State(state): State<AppState>,
    Extension(user_id): Extension<i64>, // User id obtenu avec le auth middleware
) -> Result<StatusCode, (StatusCode, String)> {

  print!("User logout request received... ");

  sql_delete_session(state, user_id).await?;

  println!("Done. Session deleted.");
      
  Ok(StatusCode::NO_CONTENT)  
}

