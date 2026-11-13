use crate::middleware::auth::*;
use crate::middleware::logger::*;
use crate::routes::escapade_delete::*;
use crate::routes::escapade_list::*;
use crate::routes::escapade_post::*;
use crate::routes::heartbeat_delete::*;
use crate::routes::heartbeat_post::*;
use crate::routes::index::*;
use crate::routes::upload::*;
use crate::routes::user_login::*;
use crate::routes::user_logout::*;
use crate::routes::user_register::*;
use crate::state::AppState;
use axum::extract::DefaultBodyLimit;
use axum::middleware::{from_fn, from_fn_with_state};
use axum::routing::delete;
use axum::{
    routing::{get, post},
    Router,
};
use tower_http::services::ServeDir;

pub fn create_router(state: AppState) -> Router {
    // Protected routes (requires valide Authorization header)
    let protected_routes = Router::new()
        .route("/escapades/{escapade_id}/heartbeat", post(heartbeat_post)) // Create a new heartbeat
        .route("/heartbeats/{heartbeat_id}", delete(heartbeat_delete)) // Delete a heartbeat
        .route("/escapades", post(escapade_post)) // Create (and join) a new escapade
        .route("/escapades", get(escapades_list)) // list all escapades of the user
        .route("/escapades/{escapade_id}", delete(escapade_delete)) // Delete an escapade (with cascade)
        .route("/upload/{picture_id}", post(post_picture)) // Upload in conjuction with a hb
        .route("/user/logout", delete(session_logout))
        .layer(from_fn_with_state(state.clone(), auth)); // auth middleware

    // Full router
    Router::new()
        .route("/user/new", post(register)) // Create a new user
        .route("/user/login", post(login)) // Verif + token to user
        .route("/", get(index)) // Display the index.html template (last heartbeat loaded)
        .route("/image/{picture_id}", get(get_picture))
        .merge(protected_routes)
        .nest_service("/static", ServeDir::new("static"))
        .layer(DefaultBodyLimit::max(20 * 1024 * 1024))
        .layer(from_fn(logger))
        .with_state(state)
}
