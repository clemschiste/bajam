use crate::auth;
use crate::auth::middleware::*;
use crate::escapades;
use crate::middleware::logger::*;
use crate::middleware::origin::middleware_origin;
use crate::routes::paths;
use crate::routes::upload::*;
use crate::state::AppState;
use crate::views::dashboard::*;
use axum::extract::DefaultBodyLimit;
use axum::middleware::{from_fn, from_fn_with_state};
use axum::{
    routing::{get, post},
    Router,
};
use tower_http::services::ServeDir;

pub fn create_router(state: AppState) -> Router {
    // Protected routes (requires valide Authorization header)
    let protected_routes = Router::new()
        .route(paths::UPLOAD, post(post_picture)) // Upload in conjuction with a hb
        .route(paths::DASHBOARD, get(dashboard_view)) // Display the dashboard.html template (last heartbeat loaded)
        .route(paths::ROOT, get(|| async {})) // redirect either to dashboard or login (see auth middleware)
        .merge(auth::protected_routes())
        .merge(escapades::protected_routes())
        .layer(from_fn_with_state(state.clone(), middleware_origin)) // origin middleware
        .layer(from_fn_with_state(state.clone(), middleware_auth)); // auth middleware

    // Full router
    Router::new()
        .merge(auth::public_routes())
        .route(paths::IMAGE, get(get_picture))
        .merge(protected_routes)
        .nest_service(paths::STATIC, ServeDir::new("static"))
        .layer(DefaultBodyLimit::max(20 * 1024 * 1024))
        .layer(from_fn(middleware_logger))
        .with_state(state)
}
