use bajam_common::models::heartbeat::Heartbeat;
use sqlx::SqlitePool;

/// List heartbeat of an escapade.
pub async fn sql_list_heartbeats(
  db: &SqlitePool,
  user_id: i64,
  escapade_id: i64,
) -> Result<Vec<Heartbeat>, sqlx::Error> {

  let list = sqlx::query_as!(
      Heartbeat,
      r#"
          SELECT *
          FROM heartbeats
          WHERE
            escapade_id = ?
            AND user_id = ?
          LIMIT 500
      "#,
      escapade_id,
      user_id
    )
    .fetch_all(db)
    .await?;

    Ok(list)

}
