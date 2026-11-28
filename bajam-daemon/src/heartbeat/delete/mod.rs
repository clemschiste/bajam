pub mod handler;
mod queries;
pub mod service;

use crate::routes::paths;
use axum::routing::delete;
use axum::Router;

use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new().route(paths::HEARTBEAT, delete(handler::heartbeat_delete)) // create via json
}
