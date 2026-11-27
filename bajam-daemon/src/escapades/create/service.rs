use sqlx::SqlitePool;

use crate::escapades::error::EscapadeServiceError;

use super::queries::sql_insert_escapade;


pub async fn create_escapade(db: &SqlitePool, user_id: i64, escapade_name: String) -> Result<(), EscapadeServiceError> {
  sql_insert_escapade(db, user_id, escapade_name)
    .await
    .map_err(|e| EscapadeServiceError::Internal(e.to_string()))?;

  Ok(())
}

// later
// - extend escapade (add user to a validation_list)
// - join escapade (join an escapade provided that you are on the validation_list)
