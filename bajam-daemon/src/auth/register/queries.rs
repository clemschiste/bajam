use sqlx::SqlitePool;

pub async fn sql_insert_user(db: &SqlitePool, username: &str, password_hash: &str) -> sqlx::Result<()> {
  sqlx::query("INSERT into users (username, password_hash) VALUES (?, ?)")
    .bind(username)
    .bind(password_hash)
    .execute(db)
    .await?;
  Ok(())
}
