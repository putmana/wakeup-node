use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

use crate::core::error::CoreError;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error(transparent)]
    Internal(#[from] CoreError),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            Self::Internal(err) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{err}")),
        }
        .into_response()
    }
}
