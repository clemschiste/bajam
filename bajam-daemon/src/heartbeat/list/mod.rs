pub mod handler;
mod queries;
pub mod service;

use crate::routes::paths;
use axum::routing::get;
use axum::Router;

use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            paths::API_ESCAPADE_HEARTBEAT,
            get(handler::heartbeat_list_api),
        )
        .route(paths::ESCAPADE_HEARTBEAT, get(handler::heartbeat_list_api))
}
