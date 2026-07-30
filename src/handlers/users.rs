use axum::extract::Path;

use crate::{
    common::{
        extractors::AppJson,
        response::{ApiResponse, ApiResult, EmptyData},
    },
    models::users::{CreateUser, UpdateUser, User},
    routes::users::UserRouterPaths,
    services::users::{create_user, delete_user, get_user_detail, get_user_list, update_user},
};

#[utoipa::path(
    get,
    path = UserRouterPaths::List.full_path(),
    responses(
        (status = 200, description = "查询成功", body = ApiResponse<Vec<User>>)
    ),
    tag = "用户管理",
    summary = "用户列表"
)]
pub async fn list_user_handler() -> ApiResult<Vec<User>> {
    let users = get_user_list()?;
    Ok(ApiResponse::ok(users))
}

#[utoipa::path(
    get,
    path = UserRouterPaths::Detail.full_path(),
    params(
        ("id" = u64, Path, description = "用户 ID", example = 1)
    ),
    responses(
        (status = 200, description = "查询成功", body = ApiResponse<User>)
    ),
    tag = "用户管理",
    summary = "用户详情"
)]
pub async fn detail_user_handler(Path(id): Path<u64>) -> ApiResult<User> {
    let user = get_user_detail(id)?;
    Ok(ApiResponse::ok(user))
}

#[utoipa::path(
    post,
    path = UserRouterPaths::Create.full_path(),
    request_body = CreateUser,
    responses(
        (status = 200, description = "新增成功", body = ApiResponse<User>),
        (status = 401, description = "Token 无效或已过期")
    ),
    security(
        ("bearerAuth" = [])
    ),
    tag = "用户管理",
    summary = "新增用户"
)]
pub async fn create_user_handler(AppJson(body): AppJson<CreateUser>) -> ApiResult<User> {
    let user = create_user(body)?;
    Ok(ApiResponse::ok(user))
}

#[utoipa::path(
    put,
    path = UserRouterPaths::Update.full_path(),
    params(
        ("id" = u64, Path, description = "用户 ID", example = 1)
    ),
    request_body = UpdateUser,
    responses(
        (status = 200, description = "更新成功", body = ApiResponse<User>)
    ),
    tag = "用户管理",
    summary = "更新用户"
)]
pub async fn update_user_handler(
    Path(id): Path<u64>,
    AppJson(body): AppJson<UpdateUser>,
) -> ApiResult<User> {
    let user = update_user(id, body)?;
    Ok(ApiResponse::ok(user))
}

#[utoipa::path(
    delete,
    path = UserRouterPaths::Delete.full_path(),
    params(
        ("id" = u64, Path, description = "用户 ID", example = 1)
    ),
    responses(
        (status = 200, description = "删除成功", body = ApiResponse<EmptyData>)
    ),
    tag = "用户管理",
    summary = "删除用户"
)]
pub async fn delete_user_handler(Path(id): Path<u64>) -> ApiResult<()> {
    delete_user(id)?;
    Ok(ApiResponse::ok_msg((), "用户删除成功"))
}
