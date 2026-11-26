use bajam_common::models::session::Session;
use chrono::{Duration, Utc};
use sha2::{Digest, Sha256};
use sqlx::SqlitePool;
use uuid::Uuid;

use super::queries::*;
use crate::auth::AuthServiceError;

fn sha256_hash(value: &str) -> String {
    hex::encode(Sha256::digest(value.as_bytes()))
}

/// Creates a session for the user, returns the raw token (only its sha256 is stored).
pub async fn create(db: &SqlitePool, user_id: i64) -> Result<String, AuthServiceError> {
    let token = Uuid::new_v4().to_string();
    let expires_at = Utc::now().naive_utc() + Duration::days(30);
    sql_insert_session(db, &sha256_hash(&token), user_id, expires_at).await?;
    Ok(token)
}

/// Valid session for token, None if unknown or expired (expired ones get deleted).
pub async fn from_token(db: &SqlitePool, token: &str) -> Result<Option<Session>, AuthServiceError> {
    let Some(session) = sql_find_session_by_hash(db, &sha256_hash(token)).await? else {
        return Ok(None);
    };
    if session.expires_at < Utc::now().naive_utc() {
        delete(db, session.user_id, session.id).await?;
        return Ok(None);
    }
    Ok(Some(session))
}

pub async fn delete(db: &SqlitePool, user_id: i64, session_id: i64) -> Result<(), AuthServiceError> {
    Ok(sql_delete_user_session(db, user_id, session_id).await?)
}

pub async fn delete_all(db: &SqlitePool, user_id: i64) -> Result<(), AuthServiceError> {
    Ok(sql_delete_all_user_sessions(db, user_id).await?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::test_support::test_db;

    #[tokio::test]
    async fn created_session_is_found() {
        let db = db_with_user().await;
        let token = create(&db, 1).await.unwrap();

        let session = from_token(&db, &token).await.unwrap();

        assert_eq!(session.map(|s| s.user_id), Some(1));
    }

    #[tokio::test]
    async fn delete_invalidates_session() {
        let db = db_with_user().await;
        let token = create(&db, 1).await.unwrap();
        let session = from_token(&db, &token).await.unwrap().unwrap();

        delete(&db, session.user_id, session.id).await.unwrap();

        assert!(from_token(&db, &token).await.unwrap().is_none());
    }

    #[tokio::test]
    async fn delete_all_invalidates_every_session() {
        let db = db_with_user().await;
        let t1 = create(&db, 1).await.unwrap();
        let t2 = create(&db, 1).await.unwrap();

        delete_all(&db, 1).await.unwrap();

        assert!(from_token(&db, &t1).await.unwrap().is_none());
        assert!(from_token(&db, &t2).await.unwrap().is_none());
    }

    #[tokio::test]
    async fn expired_session_rejected() {
        let db = db_with_user().await;
        let token = create(&db, 1).await.unwrap();
        sqlx::query("UPDATE sessions SET expires_at = ?")
            .bind(Utc::now().naive_utc() - Duration::days(1))
            .execute(&db)
            .await
            .unwrap();

        let session = from_token(&db, &token).await.unwrap();

        assert!(session.is_none());
    }

    // In-memory DB with one user (id 1), sessions reference users.
    async fn db_with_user() -> SqlitePool {
        let db = test_db().await;
        sqlx::query("INSERT INTO users (username, password_hash) VALUES ('user', 'hash')")
            .execute(&db)
            .await
            .unwrap();
        db
    }
}
