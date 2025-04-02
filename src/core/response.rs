use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

#[derive(Serialize)]
pub struct R<T: Serialize> {
    code: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
}

impl<T: Serialize> R<T> {
    pub fn success(data: T) -> Self {
        Self {
            code: 200,
            message: None,
            data: Some(data),
        }
    }

    pub fn success_empty() -> Self {
        Self {
            code: 200,
            message: None,
            data: None,
        }
    }

    pub fn error(code: u16, message: &str) -> Self {
        Self {
            code,
            message: Some(message.to_string()),
            data: None,
        }
    }
}

impl<T: Serialize> IntoResponse for R<T> {
    fn into_response(self) -> Response {
        let status = if self.code < 300 {
            StatusCode::OK
        } else {
            StatusCode::BAD_REQUEST
        };

        (status, Json(self)).into_response()
    }
}

impl<T: Serialize> From<T> for R<T> {
    fn from(data: T) -> Self {
        Self {
            code: 200,
            message: None,
            data: Some(data),
        }
    }
}
