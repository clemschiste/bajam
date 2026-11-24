use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Session {
    pub id: i64,
    pub token_hash: String,
    pub user_id: i64,
    pub expires_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
}
