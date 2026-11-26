use axum::Router;
use axum::http::StatusCode;
use bajam_daemon::config::AppConfig;
use bajam_daemon::state::AppState;
use bajam_daemon::routes::router::create_router;

mod common;
use common::*;


#[tokio::test]
// Scenario create user, login, logout
pub async fn login_logout() {
    let app = test_app().await;

    let register = test_register_user("user", "secret", app.clone()).await;
    let login = test_login_user("user", "secret", app.clone()).await;
    let token = test_extract_token_on_login(login).await;
    let logout = test_logout_user(&token, app.clone()).await;

    assert_eq!(register.status(), StatusCode::NO_CONTENT);
    assert_eq!(logout.status(), StatusCode::NO_CONTENT);
}

#[tokio::test]
// Scenario create user, wrong login
pub async fn register_and_wrong_login() {
    let app = test_app().await;

    let register = test_register_user("user", "secret", app.clone()).await;
    let login = test_login_user("user", "wrong_secret", app.clone()).await;

    assert_eq!(register.status(), StatusCode::NO_CONTENT);
    assert_eq!(login.status(), StatusCode::UNAUTHORIZED);
}

// Fresh app: in-memory DB, empty config.
async fn test_app() -> Router {
    let state = AppState::new(test_db().await, AppConfig::empty()).await.unwrap();
    create_router(state)
}
