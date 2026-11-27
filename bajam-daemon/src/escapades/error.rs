use axum::http::StatusCode;

#[derive(Debug, PartialEq)]
pub enum EscapadeServiceError {
    AccessDenied, // For "joining" an escapade
    Internal(String),
}

impl From<sqlx::Error> for EscapadeServiceError {
    fn from(e: sqlx::Error) -> Self {
        eprintln!("Database error {e}");
        Self::Internal(format!("Database error : {e}"))
    }
}

pub fn to_http(e: EscapadeServiceError) -> (StatusCode, String) {
    match e {
        EscapadeServiceError::AccessDenied => (
            StatusCode::UNAUTHORIZED,
            "Access denied to this escapade.".into(),
        ),
        EscapadeServiceError::Internal(m) => (StatusCode::INTERNAL_SERVER_ERROR, m),
    }
}
