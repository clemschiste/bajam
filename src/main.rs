use axum::extract::DefaultBodyLimit;
use axum::middleware::{from_fn, from_fn_with_state};
use axum::{Router, routing::{get, post}};
use tower_http::services::ServeDir;
use dotenv::dotenv;
use std::env;

mod config;
use crate::config::get_config_from_args;

mod state;
use state::{AppState, db_connect};

mod api;
use crate::api::heartbeat_post::*;
use crate::api::index::*;
use crate::api::upload::*;
use crate::api::user::*;
use api::logger;

// Serveur
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    
    dotenv().ok(); // Reads the .env file
    let config = get_config_from_args()?;
    let database_url = env::var("DATABASE_URL")?;
    let db = db_connect(&database_url).await?;

    // load db + history    
    let state = AppState::new(db).await?;

    let protected_routes = Router::new()
        .route("/heartbeat", post(heartbeat_post)) // Create a new heartbeat
        .route("/upload/{picture_id}", post(post_picture)) // Upload in conjuction with a hb
        .layer(from_fn_with_state(state.clone(), auth)); // auth middleware
        
    
    let app = Router::new()
        .route("/user/new", post(register_user)) // Create a new user
        .route("/user/login", post(user_login)) // Verif + token to user
        .route("/", get(index)) // Display the index.html template (last heartbeat loaded)
        .route("/image/{picture_id}", get(get_picture))
        .merge(protected_routes)
        .nest_service("/static", ServeDir::new("static"))
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
