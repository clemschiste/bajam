use sqlx::SqlitePool;

use super::queries::sql_insert_user;
use crate::auth::{AuthServiceError, Credentials};

pub async fn register(db: &SqlitePool, credentials: &Credentials) -> Result<(), AuthServiceError> {
    let hash = bcrypt::hash(credentials.password(), 10).map_err(|e| AuthServiceError::Internal(e.to_string()))?;
    Ok(sql_insert_user(db, credentials.username(), &hash).await?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::test_support::test_db;

    fn creds(u: &str, p: &str) -> Credentials {
        Credentials::new(u, p).unwrap()
    }

    #[tokio::test]
    async fn register_creates_user() {
        let db = test_db().await;

        let res = register(&db, &creds("user", "secret")).await;

        assert_eq!(res, Ok(()));
    }
}
