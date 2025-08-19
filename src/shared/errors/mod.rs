use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use sea_orm::DbErr;
use std::error::Error;
use thiserror::Error;
use crate::core::response::R;

#[derive(Debug, Error)]
pub enum RhyonError {
    #[error("数据库错误: {0}")]
    Database(#[from] DbErr),
    
    #[error("资源未找到")]
    NotFound,
    
    #[error("验证错误: {0}")]
    Validation(String),
    
    #[error("服务器错误: {0}")]
    ServerError(String),
    
    #[error("领域错误: {0}")]
    Domain(String),
}

impl From<Box<dyn Error>> for RhyonError {
    fn from(err: Box<dyn Error>) -> Self {
        RhyonError::ServerError(err.to_string())
    }
}

impl IntoResponse for RhyonError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            RhyonError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, "数据库操作失败"),
            RhyonError::NotFound => (StatusCode::NOT_FOUND, "请求的资源不存在"),
            RhyonError::Validation(_) => (StatusCode::BAD_REQUEST, "请求参数验证失败"),
            RhyonError::ServerError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "服务器内部错误"),
            RhyonError::Domain(_) => (StatusCode::BAD_REQUEST, "业务规则验证失败"),
        };

        let body = R::<()>::error(status.as_u16(), &message);

        (status, body).into_response()
    }
}