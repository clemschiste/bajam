use bajam_common::models::{session::Session, user::User};
use chrono::NaiveDateTime;
use sqlx::SqlitePool;

use crate::auth::{SessionRepo, UserRepo};

#[derive(Clone)]
pub struct SqliteUsers(pub SqlitePool);

#[derive(Clone)]
pub struct SqliteSessions(pub SqlitePool);

impl UserRepo for SqliteUsers {
    async fn find_by_username(&self, username: &str) -> anyhow::Result<Option<User>> {
        Ok(sqlx::query_as!(User, "SELECT * FROM users WHERE username = $1", username)
            .fetch_optional(&self.0)
            .await?)
    }

    async fn insert(&self, username: &str, password_hash: &str) -> anyhow::Result<()> {
        sqlx::query("INSERT into users (username, password_hash) VALUES (?, ?)")
            .bind(username)
            .bind(password_hash)
            .execute(&self.0)
            .await?;
        Ok(())
    }
}

impl SessionRepo for SqliteSessions {
    async fn insert(&self, token_hash: &str, user_id: i64, expires_at: NaiveDateTime) -> anyhow::Result<()> {
        sqlx::query("INSERT INTO sessions (token_hash, user_id, expires_at) VALUES (?, ?, ?)")
            .bind(token_hash)
            .bind(user_id)
            .bind(expires_at)
            .execute(&self.0)
            .await?;
        Ok(())
    }

    async fn find_by_hash(&self, hash: &str) -> anyhow::Result<Option<Session>> {
        Ok(sqlx::query_as!(
            Session,
            r#"
          SELECT id, user_id, token_hash, expires_at
          FROM sessions
          WHERE token_hash = $1
        "#,
            hash
        )
        .fetch_optional(&self.0)
        .await?)
    }

    async fn delete(&self, user_id: i64, session_id: i64) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM sessions WHERE user_id = (?) AND id = (?)")
            .bind(user_id)
            .bind(session_id)
            .execute(&self.0)
            .await?;
        Ok(())
    }

    async fn delete_all(&self, user_id: i64) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM sessions WHERE user_id = (?)")
            .bind(user_id)
            .execute(&self.0)
            .await?;
        Ok(())
    }
}
