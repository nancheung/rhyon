use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use std::error::Error;

#[derive(Debug)]
pub enum RhyonError {
    NotFound,
    Validation(String),
    ServerError(String),
}

#[derive(Serialize)]
struct R {
    code: u16,
    message: String,
}

impl IntoResponse for RhyonError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            RhyonError::NotFound => (StatusCode::NOT_FOUND, "Not Found".to_string()),
            RhyonError::Validation(msg) => (StatusCode::BAD_REQUEST, msg),
            RhyonError::ServerError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        let body = R {
            code: status.as_u16(),
            message,
        };

        (status, axum::Json(body)).into_response()
    }
}

impl From<Box<dyn Error>> for RhyonError {
    fn from(err: Box<dyn Error>) -> Self {
        RhyonError::ServerError(err.to_string())
    }
}
