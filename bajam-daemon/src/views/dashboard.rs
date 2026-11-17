use askama::Template;
use axum::{Extension, response::Html};
use axum::extract::State;
use axum::http::StatusCode;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use crate::db::user_queries::sql_username_from_id;
use crate::state::AppState;

use bajam_common::models::heartbeat::Heartbeat;

// Index is called on root_path and serves an updated Html<String>
// based on the last heartbeat on the database
pub async fn dashboard_view(
    State(state): State<AppState>,
    Extension(user_id): Extension<i64>
) -> Result<Html<String>, (StatusCode, String)> {
    println!("Index request received");
    let username = sql_username_from_id(&state, &user_id).await?;

    // Database query, guetting the last timestamped row
    // If empty return an empty row error.
    let heartbeat = sqlx::query_as!(
        Heartbeat,
        r#"
            SELECT latitude, longitude, timestamp, description, picture_id, user_id, escapade_id
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
    let template = DashboardTemplate {
        latitude: response.latitude,
        longitude: response.longitude,
        timestamp: response.timestamp,
        description: response.description.clone().unwrap_or_default(),
        picture_id: response.picture_id.clone(),
        history_json,
        username
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
#[template(path = "dashboard.html")]
struct DashboardTemplate {
    latitude: f64,
    longitude: f64,
    timestamp: NaiveDateTime,
    description: String,
    picture_id: String,
    history_json: String,
    username: String,
}

// Heartbeat response. Inclut un historique des positions
#[derive(Serialize, Deserialize, Debug)]
pub struct HeartbeatResponse {
    pub latitude: f64,
    pub longitude: f64,
    pub timestamp: NaiveDateTime,
    pub description: Option<String>,
    pub picture_id: String,
    pub history: Vec<Heartbeat>, // Pour map libre js -> [longitude, latitude]
}
