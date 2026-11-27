use sqlx::SqlitePool;

/// Deletes the selected escapade belonging to the user (must be admin).
pub async fn sql_delete_escapade(db: &SqlitePool, user_id: i64, escapade_id: i64) -> Result<(), sqlx::Error> {

  sqlx::query(
    r#"
      DELETE FROM escapades
      WHERE id = (?)
      AND EXISTS (
        SELECT 1
        FROM escapades_map
        WHERE escapade_id = ?
        AND user_id = ?
        AND role = 'admin'
      )
    "#
  )
  .bind(escapade_id)
  .bind(escapade_id)
  .bind(user_id)
  .execute(db)
    .await?;

  Ok(())
}
