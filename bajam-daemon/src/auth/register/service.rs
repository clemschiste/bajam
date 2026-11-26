use sqlx::SqlitePool;

use super::queries::sql_insert_user;
use crate::auth::AuthServiceError;

pub async fn register(db: &SqlitePool, username: &str, password: &str) -> Result<(), AuthServiceError> {
    let hash = bcrypt::hash(password, 10).map_err(|e| AuthServiceError::Internal(e.to_string()))?;
    Ok(sql_insert_user(db, username, &hash).await?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::test_db;

    #[tokio::test]
    async fn register_creates_user() {
        let db = test_db().await;

        let res = register(&db, "user", "secret").await;

        assert_eq!(res, Ok(()));
    }
}
