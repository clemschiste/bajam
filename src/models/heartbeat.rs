use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

// Heartbeat response. Inclut un historique des positions
#[derive(Serialize, Deserialize, Debug)]
pub struct HeartbeatResponse {
    pub latitude: f64,
    pub longitude: f64,
    pub timestamp: NaiveDateTime,
    pub description: Option<String>,
    pub picture_id: Option<String>,
    pub history: Vec<Heartbeat>, // Pour map libre js -> [longitude, latitude]
}

// SQL struct
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Heartbeat {
    pub latitude: f64,
    pub longitude: f64,
    pub timestamp: NaiveDateTime,
    pub description: Option<String>,
    pub picture_id: Option<String>,
}
