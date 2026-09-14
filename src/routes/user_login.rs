use bcrypt::verify;
use axum::{Json, extract::State};
use axum::http::{StatusCode};
use uuid::Uuid;
use sha2::{Sha256, Digest};
use chrono::{Duration, Utc};
use crate::AppState;
use crate::models::user::{User, LoginUser};
use crate::models::session::LoginResponse;
use crate::db::delete_session::sql_delete_session;

// Here we want to check if the username/passord input is valid
// Then handle a UUid token that we sha256 hash in the db

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginUser>
) -> Result<Json<LoginResponse>, (StatusCode, String)> {

    // Get user info and check credentials

    let user_opt = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE username = $1",
        payload.username,
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()))?;

    let user = match user_opt {
        Some(user) => user,
        None => return Err((StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()))
    };

    let valid = verify(&payload.password, &user.password_hash)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()))?;

    if !valid {
        return Err((StatusCode::UNAUTHORIZED, "Invalid password".to_string()));
    }

    // Give token for auth and insert it into the session table hashed
    // Here we opt for Uuid::v4 sha256 hashed in the db 
    let token = Uuid::new_v4();
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
    
    Ok(
        Json(
            LoginResponse { token }
        )
    )
}
