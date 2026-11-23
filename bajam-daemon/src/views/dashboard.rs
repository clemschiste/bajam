use askama::Template;
use axum::{Extension, response::Html};
use axum::extract::State;
use axum::http::StatusCode;
use bajam_common::models::escapade::Escapade;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use crate::db::escapade_queries::sql_list_escapades;
use crate::db::user_queries::sql_username_from_id;
use crate::middleware::auth::AuthContext;
use crate::state::AppState;

use bajam_common::models::heartbeat::Heartbeat;

// Index is called on root_path and serves an updated Html<String>
// based on the last heartbeat on the database
pub async fn dashboard_view(
    State(state): State<AppState>,
    Extension(auth_context): Extension<AuthContext>
) -> Result<Html<String>, (StatusCode, String)> {
    let username = sql_username_from_id(&state, &auth_context.user_id).await?;

    // Database query, guetting the last timestamped row
    // If empty return an empty row error.
    let heartbeat_opt = sqlx::query_as!(
        Heartbeat,
        r#"
            SELECT *
            FROM heartbeats
            WHERE user_id = ?
            ORDER BY timestamp DESC
            LIMIT 1
        "#,
        auth_context.user_id
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        eprintln!("Database error {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error : {e}"),
        )
    })?;

    let escapades = sql_list_escapades(&state, &auth_context.user_id).await?;

    // Tous les heartbeats de l'utilisateur, pour le survol carte par escapade
    let user_heartbeats = sqlx::query_as!(
        Heartbeat,
        r#"
            SELECT *
            FROM heartbeats
            WHERE user_id = ?
            ORDER BY timestamp DESC
        "#,
        auth_context.user_id
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| {
        eprintln!("Database error {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error : {e}"),
        )
    })?;

    let heartbeats_json = serde_json::to_string(&user_heartbeats).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Serialization error: {e}"),
        )
    })?;

    let template = match heartbeat_opt {
        Some(heartbeat) => {
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
            DashboardTemplate {
                latitude: Some(response.latitude),
                longitude: response.longitude,
                timestamp: response.timestamp,
                description: response.description.clone().unwrap_or_default(),
                picture_id: response.picture_id.clone(),
                history_json,
                heartbeats_json,
                username,
                escapades,
            }

            
        },
        None => {
            DashboardTemplate {
                latitude: None,
                longitude: 0.0,
                timestamp: NaiveDateTime::default(),
                description: String::new(),
                picture_id: String::new(),
                history_json: "[]".to_string(),
                heartbeats_json,
                username,
                escapades,
            }
        },
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
    latitude: Option<f64>,
    longitude: f64,
    timestamp: NaiveDateTime,
    description: String,
    picture_id: String,
    history_json: String,
    heartbeats_json: String,
    username: String,
    escapades: Vec<Escapade>,
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
