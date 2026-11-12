use chrono::NaiveDateTime;

#[derive(Debug)]
#[allow(unused)]
pub struct Session {
    pub token_hash: String,
    pub user_id: i64,
    pub expires_at: NaiveDateTime,
}
