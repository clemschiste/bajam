pub mod handler;

use crate::routes::paths;
use axum::routing::post;
use axum::Router;

use crate::state::AppState;

/// Behind the auth middleware (applied by the caller).
pub fn routes() -> Router<AppState> {
    Router::new()
        .route(paths::API_LOGOUT, post(handler::session_logout))
        .route(paths::API_LOGOUT_ALL, post(handler::session_logout_all_user_sessions))
        .route(paths::LOGOUT, post(handler::session_logout_web))
        .route(paths::LOGOUT_ALL, post(handler::session_logout_all_user_sessions))
}
