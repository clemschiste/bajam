use sqlx::SqlitePool;

use crate::heartbeat::{delete::queries::sql_delete_heartbeat, error::HeartbeatServiceError};

pub async fn delete_heartbeat(db: &SqlitePool, user_id: i64, heartbeat_id: i64) -> Result<(), HeartbeatServiceError> {
  sql_delete_heartbeat(db, user_id, heartbeat_id)
    .await.map_err(|e| HeartbeatServiceError::Internal(e.to_string()))?;

  Ok(())
}
