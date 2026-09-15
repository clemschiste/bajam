use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
//
// Sql Struct
#[derive(Debug)]
#[allow(unused)]
pub struct Session {
    pub token_hash: String,
    pub user_id: i64,
    pub expires_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
}
