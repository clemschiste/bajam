pub mod handler;

use axum::routing::post;
use axum::Router;

use crate::state::AppState;

/// Behind the auth middleware (applied by the caller).
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/logout", post(handler::session_logout))
        .route("/api/logout/all", post(handler::session_logout_all_user_sessions))
        .route("/logout", post(handler::session_logout_web))
        .route("/logout/all", post(handler::session_logout_all_user_sessions))
}
