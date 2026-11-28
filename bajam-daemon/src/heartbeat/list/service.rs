use bajam_common::models::heartbeat::Heartbeat;
use sqlx::SqlitePool;

use crate::heartbeat::{error::HeartbeatServiceError, list::queries::sql_list_heartbeats};

pub async fn list_heartbeats(
    db: &SqlitePool,
    user_id: i64,
    escapade_id: i64,
) -> Result<Vec<Heartbeat>, HeartbeatServiceError> {

    sql_list_heartbeats(db, user_id, escapade_id)
        .await
        .map_err(|e| HeartbeatServiceError::Internal(e.to_string()))
}
