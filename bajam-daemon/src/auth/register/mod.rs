pub mod handler;
mod queries;
pub mod service;

use crate::routes::paths;
use axum::routing::{get, post};
use axum::Router;

use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(paths::API_REGISTER, post(handler::register)) // Create a new user
        .route(paths::REGISTER, post(handler::register_web)) // register via form view html
        .route(paths::REGISTER, get(handler::register_view)) // register page
}
