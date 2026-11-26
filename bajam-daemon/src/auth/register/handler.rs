use askama::Template;
use axum::http::StatusCode;
use axum::response::{Html, Redirect};
use axum::{extract::State, Form, Json};
use axum_extra::extract::CookieJar;

use super::service::register as register_user;
use crate::auth::login;
use crate::auth::{to_http, Credentials};
use crate::state::AppState;

#[derive(Template)]
#[template(path = "register.html")]
struct RegisterTemplate {}

pub async fn register_view() -> Result<Html<String>, (StatusCode, String)> {
    RegisterTemplate {}.render().map(Html).map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Template error: {e}"))
    })
}

// Create user, then log in (session cookie) and redirect to the dashboard
pub async fn register_web(
    State(state): State<AppState>,
    jar: CookieJar,
    Form(credentials): Form<Credentials>,
) -> Result<(CookieJar, Redirect), (StatusCode, String)> {
    register_user(&state.db, &credentials).await.map_err(to_http)?;
    let token = login::service::login(&state.db, &credentials).await.map_err(to_http)?;
    Ok(login::handler::session_redirect(jar, token))
}

// Create a user in the database with hashed_password based on post
pub async fn register(
    State(state): State<AppState>,
    Json(credentials): Json<Credentials>,
) -> Result<StatusCode, (StatusCode, String)> {
    register_user(&state.db, &credentials).await.map_err(to_http)?;
    Ok(StatusCode::NO_CONTENT)
}

#[cfg(test)]
mod tests {
    use axum::http::{header, StatusCode};
    use serde_json::json;

    use crate::auth::test_support::*;

    #[tokio::test]
    async fn register_api_returns_204() {
        let app = app().await;

        let res = send(&app, post_json("/api/register", json!({"username": "user", "password": "secret"}))).await;

        assert_eq!(res.status(), StatusCode::NO_CONTENT);
    }

    #[tokio::test]
    async fn registered_user_can_login() {
        let app = app().await;
        register_user(&app).await;

        let res = send(&app, post_json("/api/login", json!({"username": "user", "password": "secret"}))).await;

        assert_eq!(res.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn register_api_empty_password_is_rejected_before_use_case() {
        let app = app().await;

        let res = send(&app, post_json("/api/register", json!({"username": "user", "password": ""}))).await;

        assert_eq!(res.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[tokio::test]
    async fn register_web_redirects_to_dashboard() {
        let app = app().await;

        let res = send(&app, post_form("/register", "username=user&password=secret")).await;

        assert_eq!(header_str(&res, header::LOCATION), "/dashboard");
    }

    #[tokio::test]
    async fn register_web_sets_session_cookie() {
        let app = app().await;

        let res = send(&app, post_form("/register", "username=user&password=secret")).await;

        assert!(header_str(&res, header::SET_COOKIE).starts_with("session_id="));
    }
}
