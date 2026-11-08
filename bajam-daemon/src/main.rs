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
use crate::routes::router::create_router;
use crate::tasks::clean::clean_expired_sessions;

// Serveur
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    
    dotenv().ok(); // Reads the .env file
    let config = get_config_from_args()?;
    let database_path = env::var("DATABASE_PATH")?;

    println!("Current dir: {:?}", std::env::current_dir()?);
    println!("Database PATH: {}", database_path);
    
    let db = db_connect(&database_path).await?;

    // Loads database + history
    let state = AppState::new(db).await?;

    // Background tasks
    tokio::spawn(clean_expired_sessions(state.clone()));
    let app = create_router(state);

    // Listening adress
    let tcp_addr = config.tcp_addr;
    let listener = tokio::net::TcpListener::bind(&tcp_addr).await?;
    println!("Serveur available at: http://{}", tcp_addr);

    axum::serve(listener, app).await?;

    Ok(())
}

