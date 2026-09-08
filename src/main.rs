use axum::extract::DefaultBodyLimit;
use axum::middleware::from_fn;
use axum::{
    Router,
    routing::{get, post},
};
use axum::{extract::Request, middleware::Next, response::Response};
use std::time::Instant;
use sqlx::{Pool, Result, Sqlite, sqlite::SqlitePoolOptions, migrate::Migrator};
use tower_http::services::ServeDir;
use dotenv::dotenv;
use std::env;
use clap::Parser;
mod config;
use config::{Args, Config};

mod api;
use crate::api::heartbeat::*;
use crate::api::upload::*;

// Serveur
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok(); // Reads the .env file
    let args = Args::parse();
    let config = Config::from_args(&args)?;
    let database_url = env::var("DATABASE_URL")?;
    let db = db_connect(&database_url).await?;

    // load db + history    
    let state = AppState::new(db).await?;
    
    let app = Router::new()
        .route("/heartbeat", get(heartbeat_get))
        .route("/heartbeat", post(heartbeat_post))
        .route("/upload", post(post_file))
        .route("/photo/{upload_id}", get(get_photo))
        .fallback_service(ServeDir::new("static"))
        .layer(DefaultBodyLimit::max(20 * 1024 * 1024))
        .layer(from_fn(logger))
        .with_state(state);

    // adresse d'écoute
    let tcp_addr = config.tcp_addr;
    let listener = tokio::net::TcpListener::bind(&tcp_addr).await?;
    println!("Serveur lancé sur http://{}", tcp_addr);

    axum::serve(listener, app).await?;

    Ok(())
}

async fn logger(req: Request, next: Next) -> Response {
    let method = req.method().clone();
    let uri    = req.uri().clone();
    let start  = Instant::now();
    // call next and get the response
    let response = next.run(req).await;
    // now we have the status code and elapsed time
    println!(
        "{} {} {} {}ms",
        method, uri,
        response.status(),
        start.elapsed().as_millis()
    );
    // prints: GET /users 200 OK 4ms
    response
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
