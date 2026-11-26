use axum::Form;
use axum::body::Body;
use axum::http::header::SET_COOKIE;
use axum::response::{IntoResponse, Redirect};
use axum_extra::extract::cookie::{Cookie, SameSite};
use axum::{Json, extract::State};
use axum::http::{Response, StatusCode};
use bajam_common::models::session::{LoginResponse};

use crate::auth::register::UserPost;
use super::to_http;
use crate::state::AppState;

// Here we want to check if the username/passord input is valid
// Then handle a UUid token that we sha256 hash in the db
pub async fn login_web(
    State(state): State<AppState>,
    Form(payload): Form<UserPost>
) -> Result<Response<Body>, (StatusCode, String)> {

    let token = state.auth.login(&payload.username, &payload.password).await.map_err(to_http)?;
    
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

pub async fn login_api(
    State(state): State<AppState>,
    Json(payload): Json<UserPost>
) -> Result<Json<LoginResponse>, (StatusCode, String)> {

    let token = state.auth.login(&payload.username, &payload.password).await.map_err(to_http)?;
    
    Ok(
        Json(
            LoginResponse { token }
        )
    )
}
