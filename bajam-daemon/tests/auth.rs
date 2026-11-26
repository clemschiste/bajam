use std::sync::Mutex;

use bajam_common::models::{session::Session, user::User};
use bajam_daemon::auth::{sha256_hash, AuthService, AuthServiceError, SessionRepo, UserRepo};
use chrono::{Duration, NaiveDateTime, Utc};

#[tokio::test]
async fn register() {
    let auth = svc();

    let res = auth.register("user", "secret").await;

    assert_eq!(res, Ok(()));
}

#[tokio::test]
async fn login() {
    let auth = svc();
    auth.register("user", "secret").await.unwrap();

    let token = auth.login("user", "secret").await.unwrap();

    assert!(auth.session_from_token(&token).await.unwrap().is_some());
}

#[tokio::test]
async fn wrong_password() {
    let auth = svc();
    auth.register("user", "secret").await.unwrap();

    let res = auth.login("user", "wrong_secret").await;

    assert_eq!(res, Err(AuthServiceError::InvalidCredentials));
}

#[tokio::test]
async fn unknown_user() {
    let auth = svc();

    let res = auth.login("ghost", "secret").await;

    assert_eq!(res, Err(AuthServiceError::InvalidCredentials));
}

#[tokio::test]
async fn logout() {
    let auth = svc();
    auth.register("user", "secret").await.unwrap();
    let token = auth.login("user", "secret").await.unwrap();
    let session = auth.session_from_token(&token).await.unwrap().unwrap();

    auth.logout(session.user_id, session.id).await.unwrap();

    assert!(auth.session_from_token(&token).await.unwrap().is_none());
}

#[tokio::test]
async fn logout_all() {
    let auth = svc();
    auth.register("user", "secret").await.unwrap();
    let t1 = auth.login("user", "secret").await.unwrap();
    let t2 = auth.login("user", "secret").await.unwrap();
    let user_id = auth.session_from_token(&t1).await.unwrap().unwrap().user_id;

    auth.logout_all(user_id).await.unwrap();

    assert!(auth.session_from_token(&t1).await.unwrap().is_none());
    assert!(auth.session_from_token(&t2).await.unwrap().is_none());
}

#[tokio::test]
async fn expired_session_rejected() {
    let sessions = FakeSessions::default();
    let expired = Utc::now().naive_utc() - Duration::days(1);
    sessions.insert(&sha256_hash("tok"), 1, expired).await.unwrap();
    let auth = AuthService::new(FakeUsers::default(), sessions);

    let res = auth.session_from_token("tok").await.unwrap();

    assert!(res.is_none());
}

// In-memory fakes of the storage traits. Real AuthService logic runs on top.
#[derive(Default)]
struct FakeUsers(Mutex<Vec<User>>);

#[derive(Default)]
struct FakeSessions(Mutex<Vec<Session>>);

impl UserRepo for FakeUsers {
    async fn find_by_username(&self, username: &str) -> anyhow::Result<Option<User>> {
        let users = self.0.lock().unwrap();
        Ok(users.iter().find(|u| u.username == username).map(|u| User {
            id: u.id,
            username: u.username.clone(),
            password_hash: u.password_hash.clone(),
        }))
    }

    async fn insert(&self, username: &str, password_hash: &str) -> anyhow::Result<()> {
        let mut users = self.0.lock().unwrap();
        let id = users.len() as i64 + 1;
        users.push(User { id, username: username.into(), password_hash: password_hash.into() });
        Ok(())
    }
}

impl SessionRepo for FakeSessions {
    async fn insert(&self, token_hash: &str, user_id: i64, expires_at: NaiveDateTime) -> anyhow::Result<()> {
        let mut s = self.0.lock().unwrap();
        let id = s.len() as i64 + 1;
        s.push(Session { id, token_hash: token_hash.into(), user_id, expires_at });
        Ok(())
    }

    async fn find_by_hash(&self, token_hash: &str) -> anyhow::Result<Option<Session>> {
        let s = self.0.lock().unwrap();
        Ok(s.iter().find(|s| s.token_hash == token_hash).map(|s| Session {
            id: s.id,
            token_hash: s.token_hash.clone(),
            user_id: s.user_id,
            expires_at: s.expires_at,
        }))
    }

    async fn delete(&self, user_id: i64, session_id: i64) -> anyhow::Result<()> {
        self.0.lock().unwrap().retain(|s| !(s.user_id == user_id && s.id == session_id));
        Ok(())
    }

    async fn delete_all(&self, user_id: i64) -> anyhow::Result<()> {
        self.0.lock().unwrap().retain(|s| s.user_id != user_id);
        Ok(())
    }
}

type Svc = AuthService<FakeUsers, FakeSessions>;

fn svc() -> Svc {
    AuthService::new(FakeUsers::default(), FakeSessions::default())
}
