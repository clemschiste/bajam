use askama::Template;
use axum::http::StatusCode;
use axum::response::{Html, Redirect};
use axum::{extract::State, Form, Json};
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use bajam_common::models::session::LoginResponse;

use super::service::login;
use crate::auth::{to_http, Credentials};
use crate::state::AppState;

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate {}

pub async fn login_view() -> Result<Html<String>, (StatusCode, String)> {
    LoginTemplate {}.render().map(Html).map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Template error: {e}"))
    })
}

pub async fn login_web(
    State(state): State<AppState>,
    jar: CookieJar,
    Form(credentials): Form<Credentials>,
) -> Result<(CookieJar, Redirect), (StatusCode, String)> {
    let token = login(&state.db, &credentials).await.map_err(to_http)?;
    Ok(session_redirect(jar, token))
}

pub async fn login_api(
    State(state): State<AppState>,
    Json(credentials): Json<Credentials>,
) -> Result<Json<LoginResponse>, (StatusCode, String)> {
    let token = login(&state.db, &credentials).await.map_err(to_http)?;
    Ok(Json(LoginResponse { token }))
}

/// Redirect to the dashboard with the session cookie set.
pub fn session_redirect(jar: CookieJar, token: String) -> (CookieJar, Redirect) {
    let cookie = Cookie::build(("session_id", token))
        .http_only(true)
        // .secure(true)
        .same_site(SameSite::Lax)
        .path("/")
        .build();

    (jar.add(cookie), Redirect::to("/dashboard"))
}
