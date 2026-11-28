use sqlx::SqlitePool;

pub async fn sql_delete_heartbeat(db: &SqlitePool, user_id: i64, heartbeat_id: i64) -> Result<(), sqlx::Error> {
  sqlx::query(
    r#"
      DELETE FROM heartbeats WHERE id = (?) AND user_id = (?)
    "#
  )
  .bind(heartbeat_id)
  .bind(user_id)
  .execute(db)
  .await?;

  Ok(())
}
