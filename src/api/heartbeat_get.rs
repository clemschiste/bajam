// Respond on GET /heartbeat -> Probably not usefull anymore

use axum::{Json, extract::State};
use axum::http::StatusCode;
use sqlx::Result;

use crate::AppState;
use super::{Heartbeat, HeartbeatResponse};

// Récupère le dernier heartbeat
pub async fn heartbeat_get(State(mut state): State<AppState>) -> Result<axum::Json<HeartbeatResponse>, (StatusCode, String)> {
    println!("Heartbeat request received");
    let heartbeat = sqlx::query_as!(
        Heartbeat,
        r#"
            SELECT latitude, longitude, timestamp, description, upload_id
            FROM heartbeats
            ORDER By timestamp DESC
            LIMIT 1
        "#
    )
    .fetch_one(&state.db)
    .await
    .map_err(|e| {
        eprintln!("Database error {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error : {e}"))
    })?;
    // If empty -> Database error row empty
    // If first heartbeat posted ->

    let previous_history = state.history.clone();
    // Ajouter une condition de push si le timestamp a changé seulement.
    if !previous_history.is_empty() && previous_history[0].timestamp != heartbeat.timestamp {
        state.history.push(heartbeat.clone());
    }
    Ok(Json(HeartbeatResponse {
        latitude: heartbeat.latitude,
        longitude: heartbeat.longitude,
        timestamp: heartbeat.timestamp,
        description: heartbeat.description,
        upload_id: heartbeat.upload_id,
        history: previous_history
     }))    
}
