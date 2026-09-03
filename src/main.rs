use std::net::SocketAddr;
use std::env;
use serde::{Deserialize, Serialize, Deserializer};
use axum::{Json, extract::State};
use axum::{
    Router,
    routing::{get, post},
};
use axum::http::StatusCode;
use sqlx::{Pool, Result, Sqlite, SqlitePool, sqlite::SqlitePoolOptions, migrate::Migrator};
use tower_http::services::ServeDir;
use dotenv::dotenv;

// Serveur

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok(); // Reads the .env file
    let database_url = env::var("DATABASE_URL")?;
    let db = db_connect(&database_url).await?;

    // load db + history    
    let state = AppState::new(db).await?;
    
    let app = Router::new()
        .route("/heartbeat", get(heartbeat_get))
        .route("/heartbeat", post(heartbeat_post))
        .fallback_service(ServeDir::new("static"))
        .with_state(state);

    // adresse d'écoute
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    println!("Serveur lancé sur http://{}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
    pub history: Vec<Heartbeat>,
}

// A voir si je souhaite tracer la ligne. Probablement pas maintenant mais il faudrait retourner dans le bon ordre
impl AppState {
    async fn new(db: SqlitePool) -> Result<Self, sqlx::Error> {
        print!("Loading history... ");
        let history = sqlx::query_as!(
            Heartbeat,
            r#"
                SELECT latitude, longitude, timestamp
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
}

// Post struct (timestamp created at INSERTION)
// Iphone inable to send floats in the json position...
#[derive(Serialize, Deserialize, Debug)]
pub struct HeartbeatPost {
    #[serde(deserialize_with = "string_or_float")]
    latitude: f64,
    #[serde(deserialize_with = "string_or_float")]
    longitude: f64,
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

// Post struct (timestamp created at INSERTION)
#[derive(Serialize, Deserialize, Debug)]
pub struct HeartbeatPostString {
    latitude: String,
    longitude: String,
}
 
// Heartbeat response. Inclut un historique des positions
#[derive(Serialize, Deserialize, Debug)]
pub struct HeartbeatResponse {
    latitude: f64,
    longitude: f64,
    timestamp: String,
    history: Vec<Heartbeat>, // Pour map libre js -> [longitude, latitude]
}

// Récupère le dernier heartbeat
pub async fn heartbeat_get(State(mut state): State<AppState>) -> Result<axum::Json<HeartbeatResponse>, (StatusCode, String)> {
    println!("Heartbeat request received");
    let heartbeat = sqlx::query_as!(
        Heartbeat,
        r#"
            SELECT latitude, longitude, timestamp
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

    let previous_history = state.history.clone();
    // Ajouter une condition de push si le timestamp a changé seulement.
    if previous_history[0].timestamp != heartbeat.timestamp {
        state.history.push(heartbeat.clone());
    }

    Ok(Json(HeartbeatResponse {
        latitude: heartbeat.latitude,
        longitude: heartbeat.longitude,
        timestamp: heartbeat.timestamp,
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
           INSERT INTO heartbeats (latitude, longitude)
           VALUES (?, ?)
       "#
    )
    .bind(payload.latitude)
    .bind(payload.longitude)
    .execute(&state.db)
    .await
    .map_err(|e| {
        eprintln!("Database error {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error : {e}"))
    })?;

    Ok(StatusCode::NO_CONTENT)
}

async fn db_connect(path: &str) -> Result<Pool<Sqlite>, sqlx::Error> {
    println!("Connecting to db at {}.", path);
    let db = SqlitePoolOptions::new()
        .max_connections(3)
        .connect(path)
        .await?;

    print!("Checking for migrations... ");
    MIGRATOR.run(&db).await?;

    println!("Done.");
    Ok(db)

}
// Embeds migrations into the compiled binary
// Empty -> default to "./migrations"
static MIGRATOR: Migrator = sqlx::migrate!();
