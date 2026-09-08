use serde::{Deserialize, Serialize, Deserializer};
use axum::{Json, extract::State};
use axum::http::StatusCode;
use sqlx::Result;
use crate::AppState;

pub async fn heartbeat_post(
    State(state): State<AppState>,
    Json(payload): Json<HeartbeatPost>
) -> Result<StatusCode, (StatusCode, String)> {

    sqlx::query(
       r#"
           INSERT INTO heartbeats (latitude, longitude, description, upload_id)
           VALUES (?, ?, ?, ?)
       "#
    )
    .bind(payload.latitude)
    .bind(payload.longitude)
    .bind(payload.description)
    .bind(payload.upload_id)
    .execute(&state.db)
    .await
    .map_err(|e| {
        eprintln!("Database error {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error : {e}"))
    })?;

    Ok(StatusCode::NO_CONTENT)
}


// Post struct (timestamp created at INSERTION)
// Iphone inable to send floats in the json position...
#[derive(Serialize, Deserialize, Debug)]
pub struct HeartbeatPost {
    #[serde(deserialize_with = "string_or_float")]
    latitude: f64,
    #[serde(deserialize_with = "string_or_float")]
    longitude: f64,
    description: Option<String>,
    upload_id: Option<String>,
}

fn string_or_float<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrFloat {
        Float(f64),
        String(String),
    }

    match StringOrFloat::deserialize(deserializer)? {
        StringOrFloat::Float(value) => Ok(value),
        StringOrFloat::String(value) => value
            .parse::<f64>()
            .map_err(serde::de::Error::custom),
    }
}


