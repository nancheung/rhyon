use crate::core::response::R;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use sea_orm::DbErr;
use std::error::Error;

#[derive(Debug)]
pub enum RhyonError {
    Database(DbErr),
    NotFound,
    Validation(String),
    ServerError(String),
}

impl IntoResponse for RhyonError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            RhyonError::Database(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
            RhyonError::NotFound => (StatusCode::NOT_FOUND, "Not Found".to_string()),
            RhyonError::Validation(msg) => (StatusCode::BAD_REQUEST, msg),
            RhyonError::ServerError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        let body = R::<()>::error(status.as_u16(), &message);

        (status, body).into_response()
    }
}

impl From<DbErr> for RhyonError {
    fn from(err: DbErr) -> Self {
        RhyonError::Database(err)
    }
}

impl From<Box<dyn Error>> for RhyonError {
    fn from(err: Box<dyn Error>) -> Self {
        RhyonError::ServerError(err.to_string())
    }
}
