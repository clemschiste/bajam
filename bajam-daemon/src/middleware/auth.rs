use axum::http::{HeaderMap, StatusCode, header};
use axum::response::{IntoResponse, Redirect};
use axum::{middleware::Next, response::Response};
use axum::extract::{State, Request};
use axum_extra::extract::CookieJar;
use sha2::{Sha256, Digest};
use bajam_common::models::session::Session;
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
    let path = req.uri().path().to_string();
    let credentials = match extract_credential(req.headers()) {
        Ok(credentials) => credentials,
        Err(_) => return Ok(Redirect::to("/login").into_response()),
    };

    // Check that the session exists in db. On none variant redirect to /login
    let opt_session = match credentials {
        AuthCredentials::Bearer(token) => {fetch_opt_session(&state, sha256_hash(token)).await?},
        AuthCredentials::SessionCookie(cookie) => {fetch_opt_session(&state, sha256_hash(cookie)).await?},
    };

    let session = match opt_session {
        Some(session) => session,
        None => return Ok(Redirect::to("/login").into_response()), 
    };

    // Check that the session is valid timewise or delete it
    if session.expires_at < chrono::Utc::now().naive_utc() {
        sql_delete_user_session(&state, &session.user_id).await.map_err(|_| AuthError::DatabaseError)?;
        return Ok(Redirect::to("/login").into_response())
    }

    req.extensions_mut().insert(session.user_id);

    // Redirect to dashboard if the session is all valid
    if path == "/" {
        return Ok(Redirect::to("/dashboard").into_response());
    }
    
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
 
pub fn sha256_hash(value: String) -> String {
    let hashed_value = Sha256::digest(value.as_bytes());
    hex::encode(hashed_value)
}

pub async fn fetch_opt_session(state: &AppState, hash: String) -> Result<Option<Session>, AuthError> {
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

    Ok(session_opt)
}
