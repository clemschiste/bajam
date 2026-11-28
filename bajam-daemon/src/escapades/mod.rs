pub mod create;
pub mod delete;
pub mod error;
pub mod list;

use axum::Router;

use crate::state::AppState;

/// Routes behind the auth middleware (applied by the caller).
pub fn protected_routes() -> Router<AppState> {
    Router::new()
        .merge(create::routes())
        .merge(list::routes())
        .merge(delete::routes())
}
