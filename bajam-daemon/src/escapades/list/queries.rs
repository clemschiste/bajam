use sqlx::SqlitePool;
use bajam_common::models::escapade::Escapade;

pub async fn sql_list_escapades(db: &SqlitePool, user_id: i64) -> Result<Vec<Escapade>, sqlx::Error> {

  let result = sqlx::query_as!(
    Escapade,
    r#"
        SELECT name, id
        FROM escapades
        INNER JOIN escapades_map
          ON escapades.id = escapades_map.escapade_id
        WHERE escapades_map.user_id = ?
        LIMIT 100
    "#,
    user_id
  )
  .fetch_all(db)
  .await?;

  Ok(result)
}
