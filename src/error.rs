use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("configuration error: {0}")]
    Config(String),
    #[error("webhook verification failed")]
    InvalidSignature,
    #[error("unsupported GitHub event: {0}")]
    UnsupportedEvent(String),
    #[error("invalid request: {0}")]
    BadRequest(String),
    #[error("github client error: {0}")]
    GitHub(String),
    #[error("llm error: {0}")]
    Llm(String),
    #[error("workspace error: {0}")]
    Workspace(String),
    #[error("internal error: {0}")]
    Internal(String),
}

#[derive(Serialize)]
struct ErrorBody {
    error: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match self {
            Self::InvalidSignature => StatusCode::UNAUTHORIZED,
            Self::UnsupportedEvent(_) => StatusCode::NOT_IMPLEMENTED,
            Self::BadRequest(_) | Self::Config(_) => StatusCode::BAD_REQUEST,
            Self::GitHub(_) | Self::Llm(_) | Self::Workspace(_) | Self::Internal(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };

        let body = Json(ErrorBody {
            error: self.to_string(),
        });

        (status, body).into_response()
    }
}

impl From<std::io::Error> for AppError {
    fn from(source: std::io::Error) -> Self {
        Self::Internal(source.to_string())
    }
}

impl From<octocrab::Error> for AppError {
    fn from(source: octocrab::Error) -> Self {
        Self::GitHub(source.to_string())
    }
}