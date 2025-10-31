use crate::utils::response::ApiResponse;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
 
};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("MongoDB error: {0}")]
    MongoError(#[from] mongodb::error::Error),
    
    #[error("MongoDB BSON error: {0}")]
    MongoBsonError(#[from] mongodb::bson::ser::Error),
    
    #[error("Authentication failed: {0}")]
    AuthError(String),
    
    #[error("Resource not found: {0}")]
    NotFound(String),
    
    #[error("Validation error: {0}")]
    ValidationError(String),
    
    #[error("Payment error: {0}")]
    PaymentError(String),
    
    #[error("Internal server error")]
    InternalError,
    #[error("S3 error: {0}")]
    S3Error(String)
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::MongoError(ref e) => {
                tracing::error!("MongoDB error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Database error".to_string())
            }
            AppError::MongoBsonError(ref e) => {
                tracing::error!("BSON error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Serialization error".to_string())
            }
            AppError::AuthError(ref msg) => (StatusCode::UNAUTHORIZED, msg.clone()),
            AppError::NotFound(ref msg) => (StatusCode::NOT_FOUND, msg.clone()),
            AppError::ValidationError(ref msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::PaymentError(ref msg) => (StatusCode::PAYMENT_REQUIRED, msg.clone()),
            AppError::InternalError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal error".to_string())
            }
            AppError::S3Error(ref msg) => {
                tracing::error!("S3 error: {:?}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, msg.clone())
            }
        };

        let body = ApiResponse::<String>::error(&error_message);

        (status, body).into_response()
    }
}

// Type alias for Results
pub type Result<T> = std::result::Result<T, AppError>;
