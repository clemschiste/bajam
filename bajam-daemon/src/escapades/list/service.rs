use sqlx::SqlitePool;

use crate::escapades::{error::EscapadeServiceError, list::queries::sql_list_escapades};
use bajam_common::models::escapade::Escapade;

pub async fn list_escapades(db: &SqlitePool, user_id: i64) -> Result<Vec<Escapade>, EscapadeServiceError> {
  sql_list_escapades(db, user_id)
    .await
    .map_err(|e| EscapadeServiceError::Internal(e.to_string()))
}
