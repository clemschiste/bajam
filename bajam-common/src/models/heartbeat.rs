use chrono::NaiveDateTime;
use serde::{Deserialize, Deserializer, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Heartbeat {
    pub latitude: f64,
    pub longitude: f64,
    pub escapade_id: i64,
    pub description: Option<String>,
    pub timestamp: NaiveDateTime, // On insertion in db
    pub picture_id: String,       // UUidv4 created in heartbeat_post
    pub user_id: i64,             // Extension heartbeat_post
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HeartbeatPost {
    #[serde(deserialize_with = "string_or_float")]
    pub latitude: f64,
    #[serde(deserialize_with = "string_or_float")]
    pub longitude: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

fn string_or_float<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrFloat {
        Float(f64),
        String(String),
    }

    match StringOrFloat::deserialize(deserializer)? {
        StringOrFloat::Float(value) => Ok(value),
        StringOrFloat::String(value) => value.parse::<f64>().map_err(serde::de::Error::custom),
    }
}

impl HeartbeatPost {
    /// Checks validity regarding the latitude and longitude.
    /// -90.0 + 90.0
    pub fn valid(&self) -> bool {
        self.latitude <= 90.0
            && self.longitude <= 90.0
            && self.latitude >= -90.0
            && self.longitude >= -90.0
    }
}

#[derive(Serialize, Deserialize)]
pub struct ImageHandle {
    pub picture_id: Uuid,
}
