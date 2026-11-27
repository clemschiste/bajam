pub mod handler;
mod queries;
pub mod service;

use crate::routes::paths;
use axum::routing::delete;
use axum::Router;

use crate::state::AppState;

/// Behind the auth middleware (applied by the caller).
pub fn routes() -> Router<AppState> {
    Router::new()
        .route(paths::ESCAPADE, delete(handler::escapade_delete)) // delete via form view
        .route(paths::API_ESCAPADE, delete(handler::escapade_delete)) // delete via json
}

// later
// can change it to manage
// add_user (if admin)
// remove_user (if admin)
