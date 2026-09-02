use std::net::SocketAddr;
use axum::{Json, extract::State};
use serde::{Deserialize, Serialize};
use axum::{
    Router,
    routing::{get, post},
};
use axum::http::StatusCode;
use sqlx::{Pool, Result, Sqlite, SqlitePool, sqlite::SqlitePoolOptions};
use tower_http::services::ServeDir;
use dotenv::dotenv;
use std::env;

// Serveur

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok(); // Reads the .env file
    let database_url = env::var("DATABASE_URL")?;
    let db = db_connect(&database_url).await?;
    
    let state = AppState::new(db);
    
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
}

impl AppState {
    fn new(db: SqlitePool) -> AppState {
        AppState { db }
    }
}

// SQL struct
#[derive(Serialize, Deserialize, Debug)]
pub struct Heartbeat {
    latitude: f64,
    longitude: f64,
    timestamp: String,
}

// Post struct (timestamp created at INSERTION)
#[derive(Serialize, Deserialize, Debug)]
pub struct HeartbeatPost {
    latitude: f64,
    longitude: f64,
}

pub async fn heartbeat_get(State(state): State<AppState>) -> Result<axum::Json<Heartbeat>, (StatusCode, String)> {
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

    Ok(Json(Heartbeat {
        latitude: heartbeat.latitude,
        longitude: heartbeat.longitude,
        timestamp: heartbeat.timestamp,
    }))
}

// Pour un post axum il faut return StatusCode
pub async fn heartbeat_post(
    State(state): State<AppState>,
    Json(payload): Json<HeartbeatPost>
) -> Result<StatusCode, (StatusCode, String)> {
    println!("Heartbeat post received");

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
    let db = SqlitePoolOptions::new()
        .max_connections(3)
        .connect(path)
        .await?;

    sqlx::migrate!("./migrations")
        .run(&db)
        .await?;

    Ok(db)

}
