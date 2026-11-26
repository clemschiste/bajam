pub mod handler;
mod queries;
pub mod service;

use axum::routing::{get, post};
use axum::Router;

use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/register", post(handler::register)) // Create a new user
        .route("/register", post(handler::register_web)) // register via form view html
        .route("/register", get(handler::register_view)) // register page
}
