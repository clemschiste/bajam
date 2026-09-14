use chrono::NaiveDateTime;
use serde::Serialize;
use uuid::Uuid;
//
// Sql Struct
#[allow(unused)]
pub struct Session {
    pub user_id: i64,
    pub expires_at: NaiveDateTime,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: Uuid,
}
