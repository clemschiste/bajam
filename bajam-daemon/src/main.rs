use state::{AppState, db_connect};
use crate::config::get_config;
use crate::routes::router::create_router;
use crate::tasks::clean::clean_expired_sessions;

mod auth;
mod config;
mod state;
mod routes;
mod tasks;
mod db;
mod middleware;
mod views;

// Server
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = get_config()?;
    let db = db_connect(&config.database.database_path).await?;

    // Loads database + history
    let state = AppState::new(db, config.clone()).await?;

    // Background tasks
    tokio::spawn(clean_expired_sessions(state.clone()));
    let app = create_router(state);

    // Listening adress

    let bind_adress = &config.network.bind_address;
    let listener = tokio::net::TcpListener::bind(bind_adress).await?;
    println!("Serveur available at: {:?}", config.network.allowed_origin);

    axum::serve(listener, app).await?;

    Ok(())
}

