use thiserror::Error;
use axum::{http::StatusCode, response::IntoResponse};
use serde_json::json;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Account not found")]
    AccountNotFound,
    #[error("Insufficient funds")]
    InsufficientFunds,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            AppError::DatabaseError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error"),
            AppError::AccountNotFound => (StatusCode::NOT_FOUND, "Account not found"),
            AppError::InsufficientFunds => (StatusCode::BAD_REQUEST, "Insufficient funds"),
        };

        (status, axum::Json(json!({ "error": error_message }))).into_response()
    }
}