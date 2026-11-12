use axum::{Json, extract::State};
use axum::http::{StatusCode};
use serde::Deserialize;
use crate::state::AppState;

// Function to create a user in the database with hashed_password based on post
pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<UserPost>
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

#[derive(Deserialize, Debug)]
pub struct UserPost {
    pub username: String,
    pub password: String,
}
