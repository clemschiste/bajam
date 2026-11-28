use sqlx::SqlitePool;
use uuid::Uuid;
use bajam_common::models::heartbeat::HeartbeatPost;

use crate::heartbeat::{create::queries::sql_insert_heartbeat, error::HeartbeatServiceError};

pub async fn create_heartbeat(db: &SqlitePool, user_id: i64, escapade_id: i64, post: HeartbeatPost) -> Result<(), HeartbeatServiceError> {
  let picture_id = Uuid::new_v4();
  sql_insert_heartbeat(db, user_id, escapade_id, picture_id, post)
    .await
    .map_err(|e| HeartbeatServiceError::Internal(e.to_string()))?;

  Ok(())
}
