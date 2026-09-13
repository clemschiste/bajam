use bcrypt::{verify};
use serde::{Deserialize, Serialize};
use axum::{Json, extract::State};
use axum::http::StatusCode;
use uuid::Uuid;
use sha2::{Sha256, Digest};
use crate::AppState;

#[derive(Deserialize, Debug)]
pub struct RegisterPost {
    pub username: String,
    pub password: String,
}

// Sql struct for query as
#[allow(unused)]
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password_hash: String,
}

// Function to create a user in the database with hashed_password based on post
pub async fn register_user(
    State(state): State<AppState>,
    Json(payload): Json<RegisterPost>
) -> Result<StatusCode, (StatusCode, String)> {

    println!("User registation request received");

    let hashed_password = bcrypt::hash(payload.password, 10)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    sqlx::query(
        r#"
            INSERT into users (username, password_hash)
            VALUES (?, ?)
        "#
    )
    .bind(payload.username)
    .bind(hashed_password)
    .execute(&state.db)
    .await
    .map_err(|e| {
        eprintln!("Database error {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error : {e}"))
    })?;

    Ok(StatusCode::NO_CONTENT)
        
}

// Data coming from user at login
#[allow(unused)]
#[derive(Serialize, Deserialize, Debug)]
pub struct LoginUser {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: Uuid,
}

// Here we want to check if the username/passord input is valid
// Then handle a UUid token that we sha256 hash in the db

#[allow(unused)]
pub async fn user_login(
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
          VALUES (?, ?, NOW() + INTERVAL '30 days')  
        "#
    )
    .bind(hash)
    .bind(user.id)
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
