use utoipa::OpenApi;

use crate::handlers::users::*;
use crate::models::users::{CreateUser, UpdateUser, User};

#[derive(OpenApi)]
#[openapi(
    paths(list_user_handler, detail_user_handler, create_user_handler, update_user_handler, delete_user_handler),
    components(schemas(User, CreateUser, UpdateUser)),
    tags((name = "用户管理", description = "用户管理相关接口"))
)]
pub struct UserApiDoc;
