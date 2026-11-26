//! Auth logic, independent from storage. Repos are traits so tests can use fakes.
use std::future::Future;
use std::sync::Arc;

use axum::http::StatusCode;
use bajam_common::models::{session::Session, user::User};
use chrono::{Duration, NaiveDateTime, Utc};
use sha2::{Digest, Sha256};
use uuid::Uuid;

pub trait UserRepo: Send + Sync {
    fn find_by_username(&self, username: &str) -> impl Future<Output = anyhow::Result<Option<User>>> + Send;
    fn insert(&self, username: &str, password_hash: &str) -> impl Future<Output = anyhow::Result<()>> + Send;
}

pub trait SessionRepo: Send + Sync {
    fn insert(&self, token_hash: &str, user_id: i64, expires_at: NaiveDateTime) -> impl Future<Output = anyhow::Result<()>> + Send;
    fn find_by_hash(&self, token_hash: &str) -> impl Future<Output = anyhow::Result<Option<Session>>> + Send;
    fn delete(&self, user_id: i64, session_id: i64) -> impl Future<Output = anyhow::Result<()>> + Send;
    fn delete_all(&self, user_id: i64) -> impl Future<Output = anyhow::Result<()>> + Send;
}

#[derive(Debug, PartialEq)]
pub enum AuthServiceError {
    InvalidCredentials,
    Internal(String),
}

impl From<anyhow::Error> for AuthServiceError {
    fn from(e: anyhow::Error) -> Self {
        eprintln!("Database error {e:#}");
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

pub struct AuthService<U, S> {
    users: Arc<U>,
    sessions: Arc<S>,
}

impl<U, S> Clone for AuthService<U, S> {
    fn clone(&self) -> Self {
        Self { users: self.users.clone(), sessions: self.sessions.clone() }
    }
}

impl<U: UserRepo, S: SessionRepo> AuthService<U, S> {
    pub fn new(users: U, sessions: S) -> Self {
        Self { users: Arc::new(users), sessions: Arc::new(sessions) }
    }

    pub async fn register(&self, username: &str, password: &str) -> Result<(), AuthServiceError> {
        let hash = bcrypt::hash(password, 10).map_err(|e| AuthServiceError::Internal(e.to_string()))?;
        Ok(self.users.insert(username, &hash).await?)
    }

    /// Checks credentials, creates a session, returns the raw token (only its sha256 is stored).
    pub async fn login(&self, username: &str, password: &str) -> Result<String, AuthServiceError> {
        let user: User = self
            .users
            .find_by_username(username)
            .await
            .map_err(|_| AuthServiceError::InvalidCredentials)?
            .ok_or(AuthServiceError::InvalidCredentials)?;

        let valid = bcrypt::verify(password, &user.password_hash).unwrap_or(false);
        if !valid {
            return Err(AuthServiceError::InvalidCredentials);
        }

        let token = Uuid::new_v4().to_string();
        let expires_at = Utc::now().naive_utc() + Duration::days(30);
        self.sessions.insert(&sha256_hash(&token), user.id, expires_at).await?;
        Ok(token)
    }

    /// Valid session for token, None if unknown or expired (expired ones get deleted).
    pub async fn session_from_token(&self, token: &str) -> Result<Option<Session>, AuthServiceError> {
        let Some(session) = self.sessions.find_by_hash(&sha256_hash(token)).await? else {
            return Ok(None);
        };
        if session.expires_at < Utc::now().naive_utc() {
            self.sessions.delete(session.user_id, session.id).await?;
            return Ok(None);
        }
        Ok(Some(session))
    }

    pub async fn logout(&self, user_id: i64, session_id: i64) -> Result<(), AuthServiceError> {
        Ok(self.sessions.delete(user_id, session_id).await?)
    }

    pub async fn logout_all(&self, user_id: i64) -> Result<(), AuthServiceError> {
        Ok(self.sessions.delete_all(user_id).await?)
    }
}
