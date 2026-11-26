use axum::routing::post;
use axum::Router;

use super::{login, logout, register};
use crate::state::AppState;

/// Routes reachable without a session.
pub fn public_routes() -> Router<AppState> {
    Router::new()
        .route("/api/register", post(register::register)) // Create a new user
        .route("/api/login", post(login::login_api)) // login via json
        .route("/register", post(register::register_web)) // register via form view html
        .route("/login", post(login::login_web)) // login via form view
}

/// Routes behind the auth middleware (applied by the caller).
pub fn protected_routes() -> Router<AppState> {
    Router::new()
        .route("/api/logout", post(logout::session_logout))
        .route("/api/logout/all", post(logout::session_logout_all_user_sessions))
        .route("/logout", post(logout::session_logout_web))
        .route("/logout/all", post(logout::session_logout_all_user_sessions))
}
