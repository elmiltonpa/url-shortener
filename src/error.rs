use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("External service error: {0}")]
    RedisError(#[from] redis::RedisError),

    #[error("URL not found")]
    NotFound,

    #[error("Invalid input: {0}")]
    ValidationError(String),

    #[error("Resource already exists")]
    Conflict,

    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    #[error("Internal server error")]
    Internal(#[from] anyhow::Error),

    #[error("The requested URL has expired and is no longer available")]
    Gone,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::DatabaseError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "A database error occurred".to_string(),
            ),
            AppError::RedisError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "A cache error occurred".to_string(),
            ),
            AppError::NotFound => (StatusCode::NOT_FOUND, self.to_string()),
            AppError::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Conflict => (StatusCode::CONFLICT, self.to_string()),
            AppError::RateLimitExceeded => (StatusCode::TOO_MANY_REQUESTS, self.to_string()),
            AppError::Gone => (StatusCode::GONE, self.to_string()),
            AppError::Internal(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "An internal server error occurred".to_string(),
            ),
        };

        let body = Json(serde_json::json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

impl AppError {
    pub fn is_unique_violation(&self) -> bool {
        match self {
            AppError::DatabaseError(sqlx::Error::Database(db_err)) => db_err.is_unique_violation(),
            _ => false,
        }
    }
}

pub type AppResult<T> = Result<T, AppError>;
