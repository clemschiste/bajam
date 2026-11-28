use axum::http::StatusCode;

#[derive(Debug, PartialEq)]
pub enum HeartbeatServiceError {
    Internal(String),
}

impl From<sqlx::Error> for HeartbeatServiceError {
    fn from(e: sqlx::Error) -> Self {
        eprintln!("Database error {e}");
        Self::Internal(format!("Database error : {e}"))
    }
}

pub fn to_http(e: HeartbeatServiceError) -> (StatusCode, String) {
    match e {
        HeartbeatServiceError::Internal(m) => (StatusCode::INTERNAL_SERVER_ERROR, m),
    }
}
