use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Heartbeat {
    pub latitude: f64,
    pub longitude: f64,
    pub timestamp: NaiveDateTime,
    pub description: Option<String>,
    pub picture_id: String,
    pub user_id: i64,
    pub escapade_id: i64,
}
