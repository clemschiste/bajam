use askama::Template;
use axum::Form;
use axum::body::Body;
use axum::http::header::SET_COOKIE;
use axum::response::{Html, IntoResponse, Redirect};
use axum_extra::extract::cookie::{Cookie, SameSite};
use bcrypt::verify;
use axum::{Json, extract::State};
use axum::http::{Response, StatusCode};
use uuid::Uuid;
use sha2::{Sha256, Digest};
use chrono::{Duration, Utc};
use bajam_common::models::session::{LoginResponse};

use crate::routes::user_register::UserPost;
use crate::state::AppState;

use bajam_common::models::user::User;

// Here we want to check if the username/passord input is valid
// Then handle a UUid token that we sha256 hash in the db

pub async fn login_web(
    State(state): State<AppState>,
    Form(payload): Form<UserPost>
) -> Result<Response<Body>, (StatusCode, String)> {

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

pub async fn login_api(
    State(state): State<AppState>,
    Json(payload): Json<UserPost>
) -> Result<Json<LoginResponse>, (StatusCode, String)> {

    let token = authenticate(&state, payload.username, payload.password).await?;
    
    Ok(
        Json(
            LoginResponse { token }
        )
    )
}

type Token = String;

async fn authenticate(state: &AppState, username: String, password: String) -> Result<Token, (StatusCode, String)> {
    let user_opt = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE username = $1",
        username,
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()))?;

    let user = match user_opt {
        Some(user) => user,
        None => return Err((StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()))
    };

    let valid = verify(password, &user.password_hash)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()))?;

    if !valid {
        return Err((StatusCode::UNAUTHORIZED, "Invalid password".to_string()));
    }

    // Give token for auth and insert it into the session table hashed
    // Here we opt for Uuid::v4 sha256 hashed in the db 
    let token = Uuid::new_v4().to_string();
    let hashed_token = Sha256::digest(token.as_bytes());
    let hash = hex::encode(hashed_token);

    sqlx::query(
        r#"
          INSERT INTO sessions (token_hash, user_id, expires_at)
          VALUES (?, ?, ?)  
        "#
    )
    .bind(hash)
    .bind(user.id)
    .bind(Utc::now().naive_utc() + Duration::days(30))
    .execute(&state.db)
    .await
    .map_err(|e| {
        eprintln!("Database error {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error : {e}"))
    })?;

    Ok(token)
}


pub async fn login_view() -> Result<Html<String>, (StatusCode, String)> {
    let template = LoginTemplate {};
    let html = template.render().map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR,
        format!("Template error: {e}"))
    })?;

    Ok(Html(html))
}

#[derive(Template)] // this will generate the code...
#[template(path = "login.html")]
struct LoginTemplate {}
