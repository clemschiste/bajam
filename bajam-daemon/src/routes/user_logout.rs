use axum::{Extension, extract::State, http::StatusCode};
use crate::middleware::auth::AuthContext;
use crate::state::AppState;
use crate::db::session_queries::sql_delete_user_session;


pub async fn session_logout(
    State(state): State<AppState>,
    Extension(auth_context): Extension<AuthContext>, // User id obtenu avec le auth middleware
) -> Result<StatusCode, (StatusCode, String)> {

  print!("User logout request received... ");

  sql_delete_user_session(&state, &auth_context.user_id).await?;

  println!("Done. Session deleted.");
      
  Ok(StatusCode::NO_CONTENT)  
}
