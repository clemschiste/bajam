use axum::Form;
use axum::body::Body;
use axum::http::header::SET_COOKIE;
use axum::response::{IntoResponse, Redirect};
use axum_extra::extract::cookie::{Cookie, SameSite};
use axum::{Json, extract::State};
use axum::http::{Response, StatusCode};
use serde::Deserialize;
use crate::routes::user_login::authenticate;
use crate::state::AppState;

// Here we want to check if the username/passord input is valid
// Then handle a UUid token that we sha256 hash in the db
pub async fn register_web(
    State(state): State<AppState>,
    Form(payload): Form<UserPost>
) -> Result<Response<Body>, (StatusCode, String)> {

    // Create user
    register_user(&payload, &state).await?;

    // Create session with authenticate and get token for session_id
    let token = authenticate(&state, payload.username, payload.password).await?;
    
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
    register_user(&payload, &state).await?;

    Ok(StatusCode::NO_CONTENT)
        
}

#[derive(Deserialize, Debug)]
pub struct UserPost {
    pub username: String,
    pub password: String,
}

async fn register_user(payload: &UserPost, state: &AppState) -> Result<StatusCode, (StatusCode, String)> {
    let hashed_password = bcrypt::hash(payload.password.clone(), 10)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    sqlx::query(
        r#"
            INSERT into users (username, password_hash)
            VALUES (?, ?)
        "#
    )
    .bind(payload.username.clone())
    .bind(hashed_password)
    .execute(&state.db)
    .await
    .map_err(|e| {
        eprintln!("Database error {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error : {e}"))
    })?;

    Ok(StatusCode::NO_CONTENT)
    
}
