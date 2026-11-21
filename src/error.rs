use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ErrorBody {
    pub error: &'static str,
    pub message: &'static str,
}

#[derive(Debug)]
pub enum ApiError {
    InvalidUrl,
    InvalidRequest,
    NotFound,
    Internal,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, body) = match self {
            ApiError::InvalidUrl => (
                StatusCode::BAD_REQUEST,
                ErrorBody {
                    error: "invalid_url",
                    message: "URL must be a valid http or https URL of at most 2048 characters",
                },
            ),
            ApiError::InvalidRequest => (
                StatusCode::BAD_REQUEST,
                ErrorBody {
                    error: "invalid_request",
                    message: "Request body must be a JSON object with a url string",
                },
            ),
            ApiError::NotFound => (
                StatusCode::NOT_FOUND,
                ErrorBody {
                    error: "not_found",
                    message: "Short code not found",
                },
            ),
            ApiError::Internal => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorBody {
                    error: "internal_error",
                    message: "Internal server error",
                },
            ),
        };
        (status, Json(body)).into_response()
    }
}

pub type AppResult<T> = Result<T, ApiError>;
