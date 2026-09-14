use axum::http::{StatusCode, header};
use axum::{middleware::Next, response::Response};
use axum::extract::{State, Request};
use sha2::{Sha256, Digest};
use crate::models::session::Session;
use crate::AppState;
use crate::db::delete_session::sql_delete_user_session;


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
    // It can be nice to separate the expires at comparaison to have an error dedicated to the session reset
    let session_opt = sqlx::query_as!(
        Session,
        r#"
          SELECT user_id, expires_at
          FROM sessions
          WHERE token_hash = $1
        "#, hash
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid session token".to_string()))?;

    let session = match session_opt {
        Some(session) => session,
        None => return Err((StatusCode::UNAUTHORIZED, "Invalid session token".to_string()))
    };

    if session.expires_at < chrono::Utc::now().naive_utc() {
        sql_delete_user_session(&state, &session.user_id).await?;
        return Err((StatusCode::UNAUTHORIZED, "Session expired".to_string()))
    }

    req.extensions_mut().insert(session.user_id);
    
    Ok(next.run(req).await)
}
