pub mod handler;
mod queries;
pub mod service;

use crate::routes::paths;
use axum::routing::{get, post};
use axum::Router;

use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(paths::API_LOGIN, post(handler::login_api)) // login via json
        .route(paths::LOGIN, post(handler::login_web)) // login via form view
        .route(paths::LOGIN, get(handler::login_view)) // login page
}
