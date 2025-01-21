use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Validation failed")]
    ValidationFailed,
    #[error("User not found")]
    NotFound,
    #[error("Database error: {0}")]
    DatabaseError(#[from] sea_orm::DbErr),
    #[error("Internal server error")]
    InternalError,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::ValidationFailed => (
                StatusCode::BAD_REQUEST,
                Json(json!({ "message": "Validation failed" })),
            ),
            AppError::NotFound => (
                StatusCode::NOT_FOUND,
                Json(json!({ "message": "Resource not found" })),
            ),
            AppError::DatabaseError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "message": "Database error", "error": err.to_string() })),
            ),
            AppError::InternalError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "message": "Internal server error" })),
            ),
        }
        .into_response()
    }
}
