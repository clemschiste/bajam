pub mod handler;
mod queries;
pub mod service;

use crate::routes::paths;
use axum::routing::post;
use axum::Router;

use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(paths::API_ESCAPADE, post(handler::escapade_post_api)) // create via json
        .route(paths::ESCAPADE, post(handler::escapade_post_web)) // create via form view
}
