use sqlx::SqlitePool;

use super::queries::sql_find_user_by_username;
use crate::auth::session;
use crate::auth::{AuthServiceError, Credentials};

/// Checks credentials, creates a session, returns the raw token.
pub async fn login(db: &SqlitePool, credentials: &Credentials) -> Result<String, AuthServiceError> {
    let user = sql_find_user_by_username(db, credentials.username())
        .await
        .map_err(|_| AuthServiceError::InvalidCredentials)?
        .ok_or(AuthServiceError::InvalidCredentials)?;

    if !bcrypt::verify(credentials.password(), &user.password_hash).unwrap_or(false) {
        return Err(AuthServiceError::InvalidCredentials);
    }

    session::service::create(db, user.id).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::register::service::register;
    use crate::auth::test_support::test_db;

    fn creds(u: &str, p: &str) -> Credentials {
        Credentials::new(u, p).unwrap()
    }

    #[tokio::test]
    async fn login_returns_token_of_a_valid_session() {
        let db = test_db().await;
        register(&db, &creds("user", "secret")).await.unwrap();

        let token = login(&db, &creds("user", "secret")).await.unwrap();

        assert!(session::service::from_token(&db, &token).await.unwrap().is_some());
    }

    #[tokio::test]
    async fn wrong_password_rejected() {
        let db = test_db().await;
        register(&db, &creds("user", "secret")).await.unwrap();

        let res = login(&db, &creds("user", "wrong_secret")).await;

        assert_eq!(res, Err(AuthServiceError::InvalidCredentials));
    }

    #[tokio::test]
    async fn unknown_user_rejected() {
        let db = test_db().await;

        let res = login(&db, &creds("ghost", "secret")).await;

        assert_eq!(res, Err(AuthServiceError::InvalidCredentials));
    }
}
