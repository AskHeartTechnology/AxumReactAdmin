use crate::common::{code::ApiCode, response::ApiResponse};

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug)]
pub enum AppError {
    BadRequest(String),
    Unauthorized(String),
    Forbidden(String),
    NotFound(String),
    Internal(String),
}

impl AppError {
    fn code(&self) -> ApiCode {
        match self {
            Self::BadRequest(_) => ApiCode::BadRequest,
            Self::Unauthorized(_) => ApiCode::Unauthorized,
            Self::Forbidden(_) => ApiCode::Forbidden,
            Self::NotFound(_) => ApiCode::NotFound,
            Self::Internal(_) => ApiCode::Internal,
        }
    }

    fn message(&self) -> &str {
        match self {
            Self::BadRequest(m)
            | Self::Unauthorized(m)
            | Self::Forbidden(m)
            | Self::NotFound(m)
            | Self::Internal(m) => m,
        }
    }

    fn status(&self) -> StatusCode {
        match self {
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            Self::Forbidden(_) => StatusCode::FORBIDDEN,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = self.status();
        let body = ApiResponse::<()>::fail_msg(self.code(), self.message());
        (status, Json(body)).into_response()
    }
}
