use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use utoipa::ToSchema;

use crate::common::{code::ApiCode, error::AppError};

#[derive(Serialize, Debug, ToSchema)]
pub struct ApiResponse<T: Serialize> {
    #[schema(value_type = i32, example = 200)]
    pub code: ApiCode,

    #[schema(example = "请求成功")]
    pub message: String,

    pub data: Option<T>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn ok(data: T) -> Self {
        Self {
            code: ApiCode::Ok,
            message: ApiCode::message(ApiCode::Ok).into(),
            data: Some(data),
        }
    }

    pub fn ok_msg(data: T, message: impl Into<String>) -> Self {
        Self {
            code: ApiCode::Ok,
            message: message.into(),
            data: Some(data),
        }
    }

    pub fn fail(code: ApiCode) -> ApiResponse<()> {
        ApiResponse {
            code,
            message: code.message().into(),
            data: None,
        }
    }

    pub fn fail_msg(code: ApiCode, message: impl Into<String>) -> ApiResponse<()> {
        ApiResponse {
            code,
            message: message.into(),
            data: None,
        }
    }
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct EmptyData {}

pub type ApiResult<T> = Result<ApiResponse<T>, AppError>;
