pub mod handler;
mod queries;
pub mod service;

use crate::routes::paths;
use axum::routing::get;
use axum::Router;

use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(paths::API_ESCAPADE, get(handler::escapades_list_api)) // create via json
        .route(paths::ESCAPADE, get(handler::escapades_list_api)) // via form view
}
