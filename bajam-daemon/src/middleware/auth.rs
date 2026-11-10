use axum::http::{HeaderMap, StatusCode, header};
use axum::response::IntoResponse;
use axum::{middleware::Next, response::Response};
use axum::extract::{State, Request};
use axum_extra::extract::CookieJar;
use sha2::{Sha256, Digest};
use crate::models::session::Session;
use crate::state::AppState;
use crate::db::session_queries::sql_delete_user_session;

enum AuthCredentials {
    Bearer(String),
    SessionCookie(String),
}

#[derive(Debug)]
pub enum AuthError {
    AmbiguousCredentials,
    MissingCredentials,
    InvalidCredentials,
    SessionExpired,
    DatabaseError
}

// for the conversion to StatusCode error
impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, message) = match self {

            AuthError::AmbiguousCredentials => (
                StatusCode::UNAUTHORIZED,
                "Multiple authentication methods provided."
            ),

            AuthError::MissingCredentials => (
                StatusCode::UNAUTHORIZED,
                "Invalid or missing credentials."
            ),

            AuthError::InvalidCredentials => (
                StatusCode::UNAUTHORIZED,
                "Invalid credentials."
            ),

            AuthError::SessionExpired => (
                StatusCode::UNAUTHORIZED,
                "Session expired."
            ),

            AuthError::DatabaseError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error"
            ),
        };

        (status, message).into_response()
    }
}

pub async fn auth(State(state): State<AppState>, mut req: Request, next: Next) -> Result<Response, AuthError> {
    let credentials = extract_credential(req.headers())?;

    let session = match credentials {
        AuthCredentials::Bearer(token) => {fetch_opt_session(&state, sha256_hash(token)).await?},
        AuthCredentials::SessionCookie(cookie) => {fetch_opt_session(&state, sha256_hash(cookie)).await?},
    };

    if session.expires_at < chrono::Utc::now().naive_utc() {
        sql_delete_user_session(&state, &session.user_id).await.map_err(|_| AuthError::DatabaseError)?;
        return Err(AuthError::SessionExpired);
    }

    req.extensions_mut().insert(session.user_id);
    
    Ok(next.run(req).await)
}

fn extract_credential(headers: &HeaderMap) -> Result<AuthCredentials, AuthError> {
    let authorization = headers.get(header::AUTHORIZATION);
    let cookies = headers.get(header::COOKIE);

    // refuse ambiguous cookies (two "auth" headers)
    if authorization.is_some() && cookies.is_some() {
        return Err(AuthError::AmbiguousCredentials);
    }

    // Authorization header extract
    if let Some(value) = authorization {
        let value = value
            .to_str()
            .map_err(|_| AuthError::InvalidCredentials)?;

        let token = value
            .strip_prefix("Bearer ")
            .ok_or(AuthError::InvalidCredentials)?;

        return Ok(AuthCredentials::Bearer(token.to_string()));
    }

    // Cookie header extract
    if let Some(_value) = cookies {
        let jar = CookieJar::from_headers(headers);
        let cookie = jar
            .get("session_id")
            .ok_or(AuthError::InvalidCredentials)?;

        return Ok(AuthCredentials::SessionCookie(cookie.value().to_string()));
    }

    Err(AuthError::MissingCredentials)
    
}
 
fn sha256_hash(value: String) -> String {
    let hashed_value = Sha256::digest(value.as_bytes());
    hex::encode(hashed_value)
}

async fn fetch_opt_session(state: &AppState, hash: String) -> Result<Session, AuthError> {
    let session_opt = sqlx::query_as!(
        Session,
        r#"
          SELECT token_hash, user_id, expires_at
          FROM sessions
          WHERE token_hash = $1
        "#, hash
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| AuthError::DatabaseError)?;

    match session_opt {
        Some(session) => Ok(session),
        None => Err(AuthError::InvalidCredentials)
    }
}
