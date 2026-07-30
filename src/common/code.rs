use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ToSchema)]
#[repr(i32)]
pub enum ApiCode {
    Ok = 200,
    BadRequest = 400,
    Unauthorized = 401,
    Forbidden = 403,
    NotFound = 404,
    Internal = 500,
}

impl ApiCode {
    pub fn as_i32(self) -> i32 {
        self as i32
    }

    pub fn message(self) -> &'static str {
        match self {
            Self::Ok => "请求成功",
            Self::BadRequest => "请求参数错误",
            Self::Unauthorized => "用户未认证",
            Self::Forbidden => "请求地址未授权",
            Self::NotFound => "请求地址不存在",
            Self::Internal => "服务内部错误",
        }
    }
}

impl Serialize for ApiCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i32(self.as_i32())
    }
}
