use bcrypt::verify;
use serde::{Deserialize, Serialize};
use axum::{Json, extract::State};
use axum::http::{StatusCode, header};
use uuid::Uuid;
use sha2::{Sha256, Digest};
use axum::{extract::Request, middleware::Next, response::Response};
use chrono::{Duration, NaiveDateTime, Utc};
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
    let expiration = Utc::now() + chrono::Duration::days(30);
        
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

// Sql Struct
#[allow(unused)]
pub struct Session {
    pub user_id: i64,
    pub expires_at: NaiveDateTime,
}
// La response fait transiter la requete vers la route méthode.
// Tout ce qu'il y a avant est prévu pour check et intercepter avec une
// Err(StatusCode::UNAUTHORIZED).
// Il faut donc lire la requete et notamment son header Authorization: Bearer *UUid*
// Décode le token, check la db -> get user_id
 
pub async fn auth(State(state): State<AppState>, mut req: Request, next: Next) -> Result<Response, (StatusCode, String)> {

    let bearer = req.headers()
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
        .ok_or((
            StatusCode::UNAUTHORIZED,
            "Invalid authorization header".to_string(),
        ))?;

    let hashed_bearer_token = Sha256::digest(bearer.as_bytes());
    let hash = hex::encode(hashed_bearer_token);

    // Check if the bearer token corresponds to a valid session
    let session_opt = sqlx::query_as!(
        Session,
        r#"
          SELECT user_id, expires_at
          FROM sessions
          WHERE token_hash = $1
              AND expires_at > CURRENT_TIMESTAMP;  
        "#, hash
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid session token".to_string()))?;

    let session = match session_opt {
        Some(session) => session,
        None => return Err((StatusCode::UNAUTHORIZED, "Invalid session token".to_string()))
    };

    req.extensions_mut().insert(session.user_id);
    
    Ok(next.run(req).await)
}
