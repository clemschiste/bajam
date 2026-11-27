use sqlx::SqlitePool;

use crate::escapades::error::EscapadeServiceError;
use crate::escapades::delete::queries::sql_delete_escapade;

pub async fn delete_escapade(db: &SqlitePool, user_id: i64, escapade_id: i64) -> Result<(), EscapadeServiceError> {
  sql_delete_escapade(db, user_id, escapade_id)
    .await
    .map_err(|e| EscapadeServiceError::Internal(e.to_string()))?;

  Ok(())
}
