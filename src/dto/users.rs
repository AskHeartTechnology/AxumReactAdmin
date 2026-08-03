use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// 新增用户
#[derive(Debug, Deserialize, ToSchema, Serialize)]
pub struct CreateUser {
    pub username: String,
    pub nickname: Option<String>,
    pub email: Option<String>,
}

/// 更新用户
#[derive(Debug, Deserialize, ToSchema, Serialize)]
pub struct UpdateUser {
    pub username: Option<String>,
    pub nickname: Option<String>,
    pub email: Option<String>,
}
