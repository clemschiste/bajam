use askama::Template;
use axum::body::Body;
use axum::http::header::SET_COOKIE;
use axum::http::{Response, StatusCode};
use axum::response::{Html, IntoResponse, Redirect};
use axum::{extract::State, Form, Json};
use axum_extra::extract::cookie::{Cookie, SameSite};
use bajam_common::models::session::LoginResponse;

use super::service::login;
use crate::auth::{to_http, UserPost};
use crate::state::AppState;

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate {}

pub async fn login_view() -> Result<Html<String>, (StatusCode, String)> {
    LoginTemplate {}.render().map(Html).map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Template error: {e}"))
    })
}

/// Redirect to the dashboard with the session cookie set.
pub fn session_redirect(token: String) -> Result<Response<Body>, (StatusCode, String)> {
    let cookie = Cookie::build(("session_id", token))
        .http_only(true)
        // .secure(true)
        .same_site(SameSite::Lax)
        .path("/")
        .build();

    let mut response = Redirect::to("/dashboard").into_response();

    response.headers_mut().insert(
        SET_COOKIE,
        cookie.to_string()
            .parse()
            .map_err(|e| (StatusCode::UNAUTHORIZED, format!("{e}")))?);

    Ok(response)
}

pub async fn login_web(
    State(state): State<AppState>,
    Form(payload): Form<UserPost>,
) -> Result<Response<Body>, (StatusCode, String)> {
    let token = login(&state.db, &payload.username, &payload.password).await.map_err(to_http)?;
    session_redirect(token)
}

pub async fn login_api(
    State(state): State<AppState>,
    Json(payload): Json<UserPost>,
) -> Result<Json<LoginResponse>, (StatusCode, String)> {
    let token = login(&state.db, &payload.username, &payload.password).await.map_err(to_http)?;
    Ok(Json(LoginResponse { token }))
}
