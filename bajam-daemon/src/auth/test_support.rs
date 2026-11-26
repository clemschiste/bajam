//! Helpers for handler -> use case integration tests: real router, in-memory sqlite.
use axum::body::Body;
use axum::http::{header, Request, Response, StatusCode};
use axum::middleware::from_fn_with_state;
use axum::Router;
use serde_json::{json, Value};
use sqlx::SqlitePool;
use tower::ServiceExt;

use super::middleware::middleware_auth;
use super::{login, logout, register};
use crate::config::AppConfig;
use crate::state::AppState;

/// Fresh in-memory DB with migrations applied.
pub async fn test_db() -> SqlitePool {
    let db = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .unwrap();
    sqlx::migrate!().run(&db).await.unwrap();
    db
}

/// Auth routes only (public + protected behind the auth middleware), on a fresh DB.
pub async fn app() -> Router {
    let state = AppState::new(test_db().await, AppConfig::empty()).await.unwrap();
    Router::new()
        .merge(login::routes())
        .merge(register::routes())
        .merge(logout::routes().layer(from_fn_with_state(state.clone(), middleware_auth)))
        .with_state(state)
}

pub async fn send(app: &Router, req: Request<Body>) -> Response<Body> {
    app.clone().oneshot(req).await.unwrap()
}

pub fn post_json(uri: &str, body: Value) -> Request<Body> {
    Request::post(uri)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(body.to_string()))
        .unwrap()
}

pub fn post_form(uri: &str, body: &str) -> Request<Body> {
    Request::post(uri)
        .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(Body::from(body.to_string()))
        .unwrap()
}

pub fn post_bearer(uri: &str, token: &str) -> Request<Body> {
    Request::post(uri)
        .header(header::AUTHORIZATION, format!("Bearer {token}"))
        .body(Body::empty())
        .unwrap()
}

pub fn post_cookie(uri: &str, token: &str) -> Request<Body> {
    Request::post(uri)
        .header(header::COOKIE, format!("session_id={token}"))
        .body(Body::empty())
        .unwrap()
}

pub fn get(uri: &str) -> Request<Body> {
    Request::get(uri).body(Body::empty()).unwrap()
}

pub async fn body_text(res: Response<Body>) -> String {
    let bytes = axum::body::to_bytes(res.into_body(), usize::MAX).await.unwrap();
    String::from_utf8(bytes.to_vec()).unwrap()
}

pub async fn body_json(res: Response<Body>) -> Value {
    let bytes = axum::body::to_bytes(res.into_body(), usize::MAX).await.unwrap();
    serde_json::from_slice(&bytes).unwrap()
}

fn header_str<'a>(res: &'a Response<Body>, name: header::HeaderName) -> &'a str {
    res.headers().get(name).unwrap().to_str().unwrap()
}

#[track_caller]
pub fn assert_status(res: &Response<Body>, expected: StatusCode) {
    assert_eq!(res.status(), expected);
}

#[track_caller]
pub fn assert_redirects_to(res: &Response<Body>, location: &str) {
    assert_eq!(header_str(res, header::LOCATION), location);
}

#[track_caller]
pub fn assert_sets_session_cookie(res: &Response<Body>) {
    assert!(header_str(res, header::SET_COOKIE).starts_with("session_id="));
}

pub async fn assert_returns_token(res: Response<Body>) {
    let token = body_json(res).await["token"].as_str().map(str::to_owned);
    assert!(token.is_some_and(|t| !t.is_empty()), "response has no token");
}

/// Registers "user"/"secret" through the API.
pub async fn register_user(app: &Router) {
    send(app, post_json("/api/register", json!({"username": "user", "password": "secret"}))).await;
}

/// Logs in "user"/"secret" (see register_user) through the API, returns the token.
pub async fn login_token(app: &Router) -> String {
    let res = send(app, post_json("/api/login", json!({"username": "user", "password": "secret"}))).await;
    body_json(res).await["token"].as_str().unwrap().to_string()
}

pub async fn assert_page_contains(res: Response<Body>, fragment: &str) {
    let html = body_text(res).await;
    assert!(html.contains(fragment), "page does not contain {fragment:?}");
}
