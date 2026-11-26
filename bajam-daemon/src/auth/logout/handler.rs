use axum::response::Redirect;
use axum::{Extension, extract::State, http::StatusCode};
use axum_extra::extract::cookie::{Cookie, SameSite};
use axum_extra::extract::CookieJar;
use crate::auth::session::middleware::AuthContext;
use crate::auth::session::service as session;
use crate::auth::to_http;
use crate::state::AppState;


pub async fn session_logout(
    State(state): State<AppState>,
    Extension(auth_context): Extension<AuthContext>, // User id obtenu avec le auth middleware
) -> Result<StatusCode, (StatusCode, String)> {

  // Session deleted in db
  session::delete(&state.db, auth_context.user_id, auth_context.session_id).await.map_err(to_http)?;
  println!("session {} deleted", auth_context.session_id);
      
  Ok(StatusCode::NO_CONTENT)  
}

pub async fn session_logout_all_user_sessions(
    State(state): State<AppState>,
    Extension(auth_context): Extension<AuthContext>, // User id obtenu avec le auth middleware
) -> Result<StatusCode, (StatusCode, String)> {

  // Session deleted in db
  session::delete_all(&state.db, auth_context.user_id).await.map_err(to_http)?;
      
  Ok(StatusCode::NO_CONTENT)  
}

pub async fn session_logout_web(
    State(state): State<AppState>,
    Extension(auth_context): Extension<AuthContext>,
    jar: CookieJar
) -> Result<(CookieJar, Redirect), (StatusCode, String)> {

    // Session deleted in db
    session::delete(&state.db, auth_context.user_id, auth_context.session_id).await.map_err(to_http)?;

    // Cookie removed from browser and redirect to login/
    let cookie_remove = Cookie::build(("session_id", ""))
      .http_only(true)
      .same_site(SameSite::Lax)
      .path("/")
      .build();

    let jar = jar.remove(cookie_remove);
    
    Ok((jar, Redirect::to("/login")))  
}
