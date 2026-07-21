use axum::extract::Path;

use crate::{
    common::{
        extractors::AppJson,
        response::{ApiResponse, ApiResult},
    },
    models::users::{CreateUser, UpdateUser, User},
    services::users::{create_user, delete_user, get_user_detail, get_user_list, update_user},
};

pub async fn list_user_handler() -> ApiResult<Vec<User>> {
    let users = get_user_list()?;
    Ok(ApiResponse::ok(users))
}

pub async fn detail_user_handler(Path(id): Path<u64>) -> ApiResult<User> {
    let user = get_user_detail(id)?;
    Ok(ApiResponse::ok(user))
}

pub async fn create_user_handler(AppJson(body): AppJson<CreateUser>) -> ApiResult<User> {
    let user = create_user(body)?;
    Ok(ApiResponse::ok(user))
}

pub async fn update_user_handler(
    Path(id): Path<u64>,
    AppJson(body): AppJson<UpdateUser>,
) -> ApiResult<User> {
    let user = update_user(id, body)?;
    Ok(ApiResponse::ok(user))
}

pub async fn delete_user_handler(Path(id): Path<u64>) -> ApiResult<()> {
    delete_user(id)?;
    Ok(ApiResponse::ok_msg((), "用户删除成功"))
}
