use serde::{Deserialize, Serialize};
use axum::{Json, extract::State};
use axum::http::StatusCode;
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
    pub username: String,
    pub hashed_password: String,
}

// Function to create a user in the database with hashed_password based on post
pub async fn register_user(
    State(state): State<AppState>,
    Json(payload): Json<RegisterPost>
) -> Result<StatusCode, (StatusCode, String)> {

    println!("User registation request received");

    let hashed_password = bcrypt::hash(payload.password, 12)
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

// Here we want to check if the username/passord input is valid 
#[allow(unused)]
pub async fn verify_user() {todo!()}
