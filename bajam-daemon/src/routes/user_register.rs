use axum::Form;
use axum::body::Body;
use axum::http::header::SET_COOKIE;
use axum::response::{IntoResponse, Redirect};
use axum_extra::extract::cookie::{Cookie, SameSite};
use axum::{Json, extract::State};
use axum::http::{Response, StatusCode};
use serde::Deserialize;
use crate::auth::to_http;
use crate::state::AppState;

// Here we want to check if the username/passord input is valid
// Then handle a UUid token that we sha256 hash in the db
pub async fn register_web(
    State(state): State<AppState>,
    Form(payload): Form<UserPost>
) -> Result<Response<Body>, (StatusCode, String)> {

    // Create user
    state.auth.register(&payload.username, &payload.password).await.map_err(to_http)?;

    // Create session and get token
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

// Function to create a user in the database with hashed_password based on post
pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<UserPost>
) -> Result<StatusCode, (StatusCode, String)> {

    // Create user
    state.auth.register(&payload.username, &payload.password).await.map_err(to_http)?;

    Ok(StatusCode::NO_CONTENT)
        
}

#[derive(Deserialize, Debug)]
pub struct UserPost {
    pub username: String,
    pub password: String,
}
