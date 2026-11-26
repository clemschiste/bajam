pub mod credentials;
pub mod error;
pub mod login;
pub mod logout;
pub mod register;
pub mod session;

use axum::Router;

use crate::state::AppState;

pub use credentials::Credentials;
pub use error::{to_http, AuthServiceError};
pub use session::middleware;

/// Routes reachable without a session.
pub fn public_routes() -> Router<AppState> {
    Router::new().merge(login::routes()).merge(register::routes())
}

/// Routes behind the auth middleware (applied by the caller).
pub fn protected_routes() -> Router<AppState> {
    logout::routes()
}

/// Fresh in-memory DB with migrations applied.
#[cfg(test)]
pub(crate) async fn test_db() -> sqlx::SqlitePool {
    let db = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .unwrap();
    sqlx::migrate!().run(&db).await.unwrap();
    db
}
