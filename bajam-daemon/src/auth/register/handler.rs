use askama::Template;
use axum::http::StatusCode;
use axum::response::{Html, Redirect};
use axum::{extract::State, Form, Json};
use axum_extra::extract::CookieJar;

use super::service::register as register_user;
use crate::auth::login;
use crate::auth::{to_http, Credentials};
use crate::state::AppState;

#[derive(Template)]
#[template(path = "register.html")]
struct RegisterTemplate {}

pub async fn register_view() -> Result<Html<String>, (StatusCode, String)> {
    RegisterTemplate {}.render().map(Html).map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Template error: {e}"))
    })
}

// Create user, then log in (session cookie) and redirect to the dashboard
pub async fn register_web(
    State(state): State<AppState>,
    jar: CookieJar,
    Form(credentials): Form<Credentials>,
) -> Result<(CookieJar, Redirect), (StatusCode, String)> {
    register_user(&state.db, &credentials).await.map_err(to_http)?;
    let token = login::service::login(&state.db, &credentials).await.map_err(to_http)?;
    Ok(login::handler::session_redirect(jar, token))
}

// Create a user in the database with hashed_password based on post
pub async fn register(
    State(state): State<AppState>,
    Json(credentials): Json<Credentials>,
) -> Result<StatusCode, (StatusCode, String)> {
    register_user(&state.db, &credentials).await.map_err(to_http)?;
    Ok(StatusCode::NO_CONTENT)
}
