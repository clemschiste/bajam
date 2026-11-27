use sqlx::SqlitePool;

pub async fn sql_insert_escapade(db: &SqlitePool, user_id: i64, escapade_name: String) -> sqlx::Result<()> {

    let mut tx = db.begin().await?;

    // Insert escapade and get id
    let escapade_id: i64 = sqlx::query_scalar(
        r#"
            INSERT INTO escapades (name)
            VALUES (?)
            RETURNING id
        "#
    )
    .bind(escapade_name)
    .fetch_one(&mut *tx)
    .await?;

    // Map created_escapade to the user
    sqlx::query(
        r#"
            INSERT INTO escapades_map (escapade_id, user_id, role)
            VALUES (?, ?, 'admin')
        "#
    )
    .bind(escapade_id)
    .bind(user_id)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(())
}
