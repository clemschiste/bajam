use chrono::NaiveDateTime;
use serde::{Deserialize, Deserializer, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Heartbeat {
    pub id: i64,
    pub latitude: f64,
    pub longitude: f64,
    pub escapade_id: i64,
    pub description: Option<String>,
    pub timestamp: NaiveDateTime, // On insertion in db
    pub picture_id: String,       // UUidv4 created in heartbeat_post
    pub user_id: i64,             // Extension heartbeat_post
}

#[derive(Serialize, Deserialize)]
pub struct ImageHandle {
    pub picture_id: Uuid,
}

#[derive(Debug, PartialEq, thiserror::Error, Deserialize)]
pub enum HeartbeatPostError {
    #[error("latitude must be in the range [-90.0, 90.0]")]
    InvalidLatitude,
    #[error("longitude must be in the range [-180.0, 180.0]")]
    InvalidLongitude,
}

/// Parsed username/password. Can only be built through `new`, or by deserializing
/// (which goes through `new`), so a `HeartbeatPost` is always valid.

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(try_from = "RawHeartbeatPost")]
pub struct HeartbeatPost {
    pub latitude: f64,
    pub longitude: f64,
    pub description: Option<String>,
}

#[derive(Deserialize)]
struct RawHeartbeatPost {
    #[serde(deserialize_with = "string_or_float")]
    latitude: f64,
    #[serde(deserialize_with = "string_or_float")]
    longitude: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
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

impl TryFrom<RawHeartbeatPost> for HeartbeatPost {
    type Error = HeartbeatPostError;

    fn try_from(raw: RawHeartbeatPost) -> Result<Self, Self::Error> {
        Self::new(raw.latitude, raw.longitude, raw.description)
    }
}

impl HeartbeatPost {
    pub fn new(
        latitude: f64,
        longitude: f64,
        description: Option<String>,
    ) -> Result<Self, HeartbeatPostError> {
        if !(-90.0..=90.0).contains(&latitude) {
            return Err(HeartbeatPostError::InvalidLatitude);
        }
        if !(-180.0..=180.0).contains(&longitude) {
            return Err(HeartbeatPostError::InvalidLongitude);
        }

        let description = description.filter(|s| !s.trim().is_empty());

        Ok(Self {
            latitude,
            longitude,
            description,
        })
    }

    pub fn latitude(&self) -> f64 {
        self.latitude
    }

    pub fn longitude(&self) -> f64 {
        self.longitude
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_heartbeat_parse() {
        let h = HeartbeatPost::new(15.000, -20.000, Some("hello".to_string())).unwrap();

        assert_eq!(
            (h.latitude, h.longitude, h.description),
            (15.000, -20.000, Some("hello".to_string()))
        );
    }

    #[test]
    fn valid_heartbeat_parse_witout_description() {
        let h = HeartbeatPost::new(15.0, -20.0, None).unwrap();

        assert_eq!(
            (h.latitude, h.longitude, h.description),
            (15.0, -20.0, None)
        );
    }

    #[test]
    fn invalid_latitude_rejected() {
        let c = HeartbeatPost::new(-170.0, 20.0, None);

        assert_eq!(c.unwrap_err(), HeartbeatPostError::InvalidLatitude);
    }

    #[test]
    fn invalid_longitude_rejected() {
        let c = HeartbeatPost::new(-20.0, 190.0, None);

        assert_eq!(c.unwrap_err(), HeartbeatPostError::InvalidLongitude);
    }

    #[test]
    fn invalid_latitude_rejected_nan() {
        let c = HeartbeatPost::new(f64::NAN, 20.0, None);

        assert_eq!(c.unwrap_err(), HeartbeatPostError::InvalidLatitude);
    }

    #[test]
    fn invalid_longitude_rejected_nan() {
        let c = HeartbeatPost::new(20.0, f64::NAN, None);

        assert_eq!(c.unwrap_err(), HeartbeatPostError::InvalidLongitude);
    }

    #[test]
    fn deserialization_validates_with_string() {
        let res: HeartbeatPost =
            serde_json::from_str(r#"{"latitude":"20.0","longitude":"20.0","description":null}"#)
                .unwrap();

        assert_eq!(res.latitude, 20.0);
        assert_eq!(res.longitude, 20.0);
        assert_eq!(res.description.as_deref(), None);
    }

    #[test]
    fn deserialization_validates_with_empty_description_string() {
        let res: HeartbeatPost =
            serde_json::from_str(r#"{"latitude":"20.0","longitude":"20.0","description":""}"#)
                .unwrap();

        assert_eq!(res.latitude, 20.0);
        assert_eq!(res.longitude, 20.0);
        assert_eq!(res.description.as_deref(), None);
    }

    #[test]
    fn deserialization_validates() {
        let res: HeartbeatPost =
            serde_json::from_str(r#"{"latitude":20.0,"longitude":20.0,"description":"hello"}"#)
                .unwrap();

        assert_eq!(res.latitude, 20.0);
        assert_eq!(res.longitude, 20.0);
        assert_eq!(res.description.as_deref(), Some("hello"));
    }
}
