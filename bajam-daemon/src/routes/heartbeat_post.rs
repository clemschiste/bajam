use axum::{Extension, Json, extract::{State, Path}};
use axum::http::StatusCode;
use sqlx::Result;
use uuid::Uuid;
use crate::{auth::middleware::AuthContext, state::AppState};
use bajam_common::models::heartbeat::{HeartbeatPost, ImageHandle};

pub async fn heartbeat_post(
    State(state): State<AppState>,
    Path(escapade_id): Path<i64>, // Plus joli si c'est le name
    Extension(auth_context): Extension<AuthContext>, // User id obtenu avec le auth middleware
    Json(payload): Json<HeartbeatPost>,
) -> Result<Json<ImageHandle>, (StatusCode, String)> {

    if !payload.valid() {
        return Err((StatusCode::UNPROCESSABLE_ENTITY, "Invalid range".to_string()))
    }

    let picture_id = Uuid::new_v4();

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
      "#
    )
    .bind(payload.latitude)
    .bind(payload.longitude)
    .bind(payload.description)
    .bind(picture_id.to_string())
    .bind(auth_context.user_id)
    .bind(escapade_id)
    .bind(escapade_id)
    .bind(auth_context.user_id)
    .execute(&state.db)
    .await
    .map_err(|e| {
        eprintln!("Database error {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error : {e}"))
    })?;

    Ok(Json(
        ImageHandle {
            picture_id
        }
      )
    )
}
