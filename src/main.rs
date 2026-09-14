use axum::extract::DefaultBodyLimit;
use axum::middleware::{from_fn, from_fn_with_state};
use axum::routing::delete;
use axum::{Router, routing::{get, post}};
use tower_http::services::ServeDir;
use dotenv::dotenv;
use std::env;

mod routes;
mod models;
mod middleware;
mod config;
mod state;
mod db;
mod tasks;

use config::get_config_from_args;
use state::{AppState, db_connect};
use crate::routes::heartbeat_post::*;
use crate::routes::index::*;
use crate::routes::upload::*;
use crate::routes::user_login::*;
use crate::routes::user_register::*;
use crate::routes::user_logout::*;
use crate::middleware::logger::*;
use crate::middleware::auth::*;
use crate::tasks::clean::clean_expired_sessions;

// Serveur
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    
    dotenv().ok(); // Reads the .env file
    let config = get_config_from_args()?;
    let database_url = env::var("DATABASE_URL")?;
    let db = db_connect(&database_url).await?;

    // Loads database + history
    let state = AppState::new(db).await?;

    // Background tasks
    tokio::spawn(clean_expired_sessions(state.clone()));

    // Protected routes (requires valide Authorization header)
    let protected_routes = Router::new()
        .route("/heartbeat", post(heartbeat_post)) // Create a new heartbeat
        .route("/upload/{picture_id}", post(post_picture)) // Upload in conjuction with a hb
        .route("/user/logout", delete(session_logout))
        .layer(from_fn_with_state(state.clone(), auth)); // auth middleware
    
    // Full router
    let app = Router::new()
        .route("/user/new", post(register)) // Create a new user
        .route("/user/login", post(login)) // Verif + token to user
        .route("/", get(index)) // Display the index.html template (last heartbeat loaded)
        .route("/image/{picture_id}", get(get_picture))
        .merge(protected_routes)
        .nest_service("/static", ServeDir::new("static"))
        .layer(DefaultBodyLimit::max(20 * 1024 * 1024))
        .layer(from_fn(logger))
        .with_state(state);

    // Listening adress
    let tcp_addr = config.tcp_addr;
    let listener = tokio::net::TcpListener::bind(&tcp_addr).await?;
    println!("Serveur available at: http://{}", tcp_addr);

    axum::serve(listener, app).await?;

    Ok(())
}

