use askama::Template;
use axum::response::Html;
use axum::extract::State;
use axum::http::StatusCode;
use chrono::NaiveDateTime;

use crate::models::heartbeat::*;
use crate::state::AppState;

// Index is called on root_path and serves an updated Html<String>
// based on the last heartbeat on the database
pub async fn index(
    State(state): State<AppState>,
) -> Result<Html<String>, (StatusCode, String)> {
    println!("Index request received");

    // Database query, guetting the last timestamped row
    // If empty return an empty row error.
    let heartbeat = sqlx::query_as!(
        Heartbeat,
        r#"
            SELECT latitude, longitude, timestamp, description, picture_id
            FROM heartbeats
            ORDER BY timestamp DESC
            LIMIT 1
        "#
    )
    .fetch_one(&state.db)
    .await
    .map_err(|e| {
        eprintln!("Database error {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error : {e}"),
        )
    })?;

    // Response is constructed from the previous sql query
    // 1) first as a struct HeartbeatResponse that includes the historic
    // 2) then as a json heartbeat_json
    let response = HeartbeatResponse {
        latitude: heartbeat.latitude,
        longitude: heartbeat.longitude,
        timestamp: heartbeat.timestamp,
        description: heartbeat.description,
        picture_id: heartbeat.picture_id,
        history: state.history.clone(),
    };

    let history_json = serde_json::to_string(&response.history)
        .map_err(|e| {
          (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Serialization error: {e}"),
          )
        }
     )?;

   // The askama template (aiming at templates/index.html) is constructed
   // with the last attributes fetched
    let template = IndexTemplate {
        latitude: response.latitude,
        longitude: response.longitude,
        timestamp: response.timestamp.clone(),
        description: response.description.clone().unwrap_or_default(),
        picture_id: response.picture_id.clone().unwrap_or_default(),
        history_json,
    };

    let html = template.render().map_err(|e| {
      (
          StatusCode::INTERNAL_SERVER_ERROR,
          format!("Template error: {e}"),
      )
    })?;

    Ok(Html(html))
    
}

#[derive(Template)] // this will generate the code...
#[template(path = "index.html")]
struct IndexTemplate {
    latitude: f64,
    longitude: f64,
    timestamp: NaiveDateTime,
    description: String,
    picture_id: String,
    history_json: String,
}
