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

#[cfg(test)]
pub(crate) mod test_support;
