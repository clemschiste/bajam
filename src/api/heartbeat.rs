use serde::{Deserialize, Serialize, Deserializer};
use axum::{Json, extract::State};
use axum::http::StatusCode;


use sqlx::{Result, SqlitePool};
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
    dbg!(&heartbeat);

    let previous_history = state.history.clone();
    dbg!(&previous_history);
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

// Il faut une première fonction qui fait la conversion

// Pour un post axum il faut return StatusCode
pub async fn heartbeat_post(
    State(state): State<AppState>,
    Json(payload): Json<HeartbeatPost>
) -> Result<StatusCode, (StatusCode, String)> {
    println!("Heartbeat post received {:?}", payload);

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

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
    pub history: Vec<Heartbeat>,
}

// A voir si je souhaite tracer la ligne. Probablement pas maintenant mais il faudrait retourner dans le bon ordre
impl AppState {
    pub async fn new(db: SqlitePool) -> Result<Self, sqlx::Error> {
        print!("Loading history... ");
        let history = sqlx::query_as!(
            Heartbeat,
            r#"
                SELECT latitude, longitude, timestamp, description, upload_id
                FROM heartbeats
                ORDER BY timestamp DESC
                LIMIT 100
            "#
        )
        .fetch_all(&db)
        .await?;

        println!("Done.");

        Ok(Self {
            db,
            history,
        })

    }
}

// SQL struct
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Heartbeat {
    latitude: f64,
    longitude: f64,
    timestamp: String,
    description: Option<String>,
    upload_id: Option<String>,
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

// Heartbeat response. Inclut un historique des positions
#[derive(Serialize, Deserialize, Debug)]
pub struct HeartbeatResponse {
    latitude: f64,
    longitude: f64,
    timestamp: String,
    description: Option<String>,
    upload_id: Option<String>,
    history: Vec<Heartbeat>, // Pour map libre js -> [longitude, latitude]
}

