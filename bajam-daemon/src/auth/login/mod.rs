pub mod handler;
mod queries;
pub mod service;

use axum::routing::{get, post};
use axum::Router;

use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/login", post(handler::login_api)) // login via json
        .route("/login", post(handler::login_web)) // login via form view
        .route("/login", get(handler::login_view)) // login page
}
