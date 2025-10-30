use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApiResponse<T>
where
    T: Serialize,
{
    pub status: String,      // "success" or "error"
    pub message: Option<String>, // Optional success/error detail
    pub data: Option<T>,     // The actual response data (if any)
}

impl<T: Serialize> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            status: "success".to_string(),
            message: None,
            data: Some(data),
        }
    }

    pub fn with_message(data: T, message: &str) -> Self {
        Self {
            status: "success".to_string(),
            message: Some(message.to_string()),
            data: Some(data),
        }
    }

    pub fn error(message: &str) -> Self {
        Self {
            status: "error".to_string(),
            message: Some(message.to_string()),
            data: None,
        }
    }
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}
