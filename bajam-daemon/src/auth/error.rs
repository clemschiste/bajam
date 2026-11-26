use axum::http::StatusCode;

#[derive(Debug, PartialEq)]
pub enum AuthServiceError {
    InvalidCredentials,
    Internal(String),
}

impl From<sqlx::Error> for AuthServiceError {
    fn from(e: sqlx::Error) -> Self {
        eprintln!("Database error {e}");
        Self::Internal(format!("Database error : {e}"))
    }
}

pub fn to_http(e: AuthServiceError) -> (StatusCode, String) {
    match e {
        AuthServiceError::InvalidCredentials => (StatusCode::UNAUTHORIZED, "Invalid credentials".into()),
        AuthServiceError::Internal(m) => (StatusCode::INTERNAL_SERVER_ERROR, m),
    }
}
