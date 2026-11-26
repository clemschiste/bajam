use axum::http::StatusCode;
use bajam_common::models::{session::Session, user::User};
use chrono::{Duration, Utc};
use sha2::{Digest, Sha256};
use sqlx::SqlitePool;
use uuid::Uuid;

#[derive(Debug, PartialEq)]
pub enum AuthServiceError {
    InvalidCredentials,
    Internal(String),
}

impl From<sqlx::Error> for AuthServiceError {
    fn from(e: sqlx::Error) -> Self {
        eprintln!("Database error {e}");
        Self::Internal(format!("Database error : {e}"))
    }
}

pub fn to_http(e: AuthServiceError) -> (StatusCode, String) {
    match e {
        AuthServiceError::InvalidCredentials => (StatusCode::UNAUTHORIZED, "Invalid credentials".into()),
        AuthServiceError::Internal(m) => (StatusCode::INTERNAL_SERVER_ERROR, m),
    }
}

pub fn sha256_hash(value: &str) -> String {
    hex::encode(Sha256::digest(value.as_bytes()))
}

#[derive(Clone)]
pub struct AuthService {
    db: SqlitePool,
}

impl AuthService {
    pub fn new(db: SqlitePool) -> Self {
        Self { db }
    }

    pub async fn register(&self, username: &str, password: &str) -> Result<(), AuthServiceError> {
        let hash = bcrypt::hash(password, 10).map_err(|e| AuthServiceError::Internal(e.to_string()))?;
        sqlx::query("INSERT into users (username, password_hash) VALUES (?, ?)")
            .bind(username)
            .bind(hash)
            .execute(&self.db)
            .await?;
        Ok(())
    }

    /// Checks credentials, creates a session, returns the raw token (only its sha256 is stored).
    pub async fn login(&self, username: &str, password: &str) -> Result<String, AuthServiceError> {
        let user = sqlx::query_as!(User, "SELECT * FROM users WHERE username = $1", username)
            .fetch_optional(&self.db)
            .await
            .map_err(|_| AuthServiceError::InvalidCredentials)?
            .ok_or(AuthServiceError::InvalidCredentials)?;

        if !bcrypt::verify(password, &user.password_hash).unwrap_or(false) {
            return Err(AuthServiceError::InvalidCredentials);
        }

        let token = Uuid::new_v4().to_string();
        sqlx::query("INSERT INTO sessions (token_hash, user_id, expires_at) VALUES (?, ?, ?)")
            .bind(sha256_hash(&token))
            .bind(user.id)
            .bind(Utc::now().naive_utc() + Duration::days(30))
            .execute(&self.db)
            .await?;
        Ok(token)
    }

    /// Valid session for token, None if unknown or expired (expired ones get deleted).
    pub async fn session_from_token(&self, token: &str) -> Result<Option<Session>, AuthServiceError> {
        let hash = sha256_hash(token);
        let session = sqlx::query_as!(
            Session,
            r#"
          SELECT id, user_id, token_hash, expires_at
          FROM sessions
          WHERE token_hash = $1
        "#,
            hash
        )
        .fetch_optional(&self.db)
        .await?;

        let Some(session) = session else { return Ok(None) };
        if session.expires_at < Utc::now().naive_utc() {
            self.logout(session.user_id, session.id).await?;
            return Ok(None);
        }
        Ok(Some(session))
    }

    pub async fn logout(&self, user_id: i64, session_id: i64) -> Result<(), AuthServiceError> {
        sqlx::query("DELETE FROM sessions WHERE user_id = (?) AND id = (?)")
            .bind(user_id)
            .bind(session_id)
            .execute(&self.db)
            .await?;
        Ok(())
    }

    pub async fn logout_all(&self, user_id: i64) -> Result<(), AuthServiceError> {
        sqlx::query("DELETE FROM sessions WHERE user_id = (?)")
            .bind(user_id)
            .execute(&self.db)
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::sqlite::SqlitePoolOptions;

    #[tokio::test]
    async fn register() {
        let auth = svc().await;

        let res = auth.register("user", "secret").await;

        assert_eq!(res, Ok(()));
    }

    #[tokio::test]
    async fn login() {
        let auth = svc().await;
        auth.register("user", "secret").await.unwrap();

        let token = auth.login("user", "secret").await.unwrap();

        assert!(auth.session_from_token(&token).await.unwrap().is_some());
    }

    #[tokio::test]
    async fn wrong_password() {
        let auth = svc().await;
        auth.register("user", "secret").await.unwrap();

        let res = auth.login("user", "wrong_secret").await;

        assert_eq!(res, Err(AuthServiceError::InvalidCredentials));
    }

    #[tokio::test]
    async fn unknown_user() {
        let auth = svc().await;

        let res = auth.login("ghost", "secret").await;

        assert_eq!(res, Err(AuthServiceError::InvalidCredentials));
    }

    #[tokio::test]
    async fn logout() {
        let auth = svc().await;
        auth.register("user", "secret").await.unwrap();
        let token = auth.login("user", "secret").await.unwrap();
        let session = auth.session_from_token(&token).await.unwrap().unwrap();

        auth.logout(session.user_id, session.id).await.unwrap();

        assert!(auth.session_from_token(&token).await.unwrap().is_none());
    }

    #[tokio::test]
    async fn logout_all() {
        let auth = svc().await;
        auth.register("user", "secret").await.unwrap();
        let t1 = auth.login("user", "secret").await.unwrap();
        let t2 = auth.login("user", "secret").await.unwrap();
        let user_id = auth.session_from_token(&t1).await.unwrap().unwrap().user_id;

        auth.logout_all(user_id).await.unwrap();

        assert!(auth.session_from_token(&t1).await.unwrap().is_none());
        assert!(auth.session_from_token(&t2).await.unwrap().is_none());
    }

    #[tokio::test]
    async fn expired_session_rejected() {
        let auth = svc().await;
        auth.register("user", "secret").await.unwrap();
        let token = auth.login("user", "secret").await.unwrap();
        sqlx::query("UPDATE sessions SET expires_at = ?")
            .bind(Utc::now().naive_utc() - Duration::days(1))
            .execute(&auth.db)
            .await
            .unwrap();

        let res = auth.session_from_token(&token).await.unwrap();

        assert!(res.is_none());
    }

    // Fresh service on an in-memory DB with migrations applied.
    async fn svc() -> AuthService {
        let db = SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .unwrap();
        sqlx::migrate!().run(&db).await.unwrap();
        AuthService::new(db)
    }
}
