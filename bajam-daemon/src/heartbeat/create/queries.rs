use sqlx::SqlitePool;
use uuid::Uuid;
use bajam_common::models::heartbeat::HeartbeatPost;

pub async fn sql_insert_heartbeat(
    db: &SqlitePool,
    user_id: i64,
    escapade_id: i64,
    picture_id: Uuid,
    post: HeartbeatPost,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO heartbeats (
            latitude,
            longitude,
            description,
            picture_id,
            user_id,
            escapade_id
        )
        SELECT ?, ?, ?, ?, ?, ?
        WHERE EXISTS (
            SELECT 1
            FROM escapades_map
            WHERE escapade_id = ?
            AND user_id = ?
        )
      "#,
    )
    .bind(post.latitude)
    .bind(post.longitude)
    .bind(post.description)
    .bind(picture_id.to_string())
    .bind(user_id)
    .bind(escapade_id)
    .bind(escapade_id)
    .bind(user_id)
    .execute(db)
    .await?;

    Ok(())
}
