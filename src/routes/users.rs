use axum::{
    Router,
    routing::{delete, get, post, put},
};
use const_format::concatcp;

use crate::handlers::users::{
    create_user_handler, delete_user_handler, detail_user_handler, list_user_handler,
    update_user_handler,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserRouterPaths {
    Prefix,
    List,
    Detail,
    Create,
    Update,
    Delete,
}

impl UserRouterPaths {
    pub const fn path(self) -> &'static str {
        match self {
            Self::Prefix => "/users",
            Self::List => "/list",
            Self::Detail => "/detail/{id}",
            Self::Create => "/create",
            Self::Update => "/update/{id}",
            Self::Delete => "/delete/{id}",
        }
    }

    pub const fn full_path(self) -> &'static str {
        match self {
            Self::Prefix => Self::Prefix.path(),
            Self::List => concatcp!(UserRouterPaths::Prefix.path(), UserRouterPaths::List.path()),
            Self::Detail => concatcp!(
                UserRouterPaths::Prefix.path(),
                UserRouterPaths::Detail.path()
            ),
            Self::Create => concatcp!(
                UserRouterPaths::Prefix.path(),
                UserRouterPaths::Create.path()
            ),
            Self::Update => concatcp!(
                UserRouterPaths::Prefix.path(),
                UserRouterPaths::Update.path()
            ),
            Self::Delete => concatcp!(
                UserRouterPaths::Prefix.path(),
                UserRouterPaths::Delete.path()
            ),
        }
    }
}

pub fn user_routes() -> Router {
    Router::new()
        .route(UserRouterPaths::List.path(), get(list_user_handler))
        .route(UserRouterPaths::Detail.path(), get(detail_user_handler))
        .route(UserRouterPaths::Create.path(), post(create_user_handler))
        .route(UserRouterPaths::Update.path(), put(update_user_handler))
        .route(UserRouterPaths::Detail.path(), delete(delete_user_handler))
}
