use axum::http::{HeaderMap, StatusCode, header};
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

fn extract_credential(headers: &HeaderMap) -> Result<AuthCredentials, (StatusCode, String)> {
    let authorization = headers.get(header::AUTHORIZATION);
    let cookies = headers.get(header::COOKIE);

    // refuse ambiguous cookies (two "auth" headers)
    if authorization.is_some() && cookies.is_some() {
        return Err((StatusCode::UNAUTHORIZED, "Ambiguous credentials".to_string()));
    }

    // Authorization header extract
    if let Some(value) = authorization {
        let value = value
            .to_str()
            .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()))?;

        let token = value
            .strip_prefix("Bearer ")
            .ok_or((StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()))?;

        return Ok(AuthCredentials::Bearer(token.to_string()));
    }

    // Cookie header extract
    if let Some(_value) = cookies {
        let jar = CookieJar::from_headers(headers);
        let cookie = jar
            .get("session_id")
            .ok_or((StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()))?;

        return Ok(AuthCredentials::SessionCookie(cookie.value().to_string()));
    }

    Err((StatusCode::UNAUTHORIZED, "Missing credentials".to_string()))
    
}
 
pub async fn auth(State(state): State<AppState>, mut req: Request, next: Next) -> Result<Response, (StatusCode, String)> {
    let credentials = extract_credential(req.headers())?;

    let session = match credentials {
        AuthCredentials::Bearer(token) => {fetch_opt_session(&state, sha256_hash(token)).await?},
        AuthCredentials::SessionCookie(cookie) => {fetch_opt_session(&state, sha256_hash(cookie)).await?},
    };

    if session.expires_at < chrono::Utc::now().naive_utc() {
        sql_delete_user_session(&state, &session.user_id).await?;
        return Err((StatusCode::UNAUTHORIZED, "Session expired".to_string()))
    }

    req.extensions_mut().insert(session.user_id);
    
    Ok(next.run(req).await)
}

fn sha256_hash(value: String) -> String {
    let hashed_value = Sha256::digest(value.as_bytes());
    hex::encode(hashed_value)
}

async fn fetch_opt_session(state: &AppState, hash: String) -> Result<Session, (StatusCode, String)> {
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
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Could not fetch session.".to_string()))?;

    match session_opt {
        Some(session) => Ok(session),
        None => Err((StatusCode::UNAUTHORIZED, "Invalid session token".to_string()))
    }
}
