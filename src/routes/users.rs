use axum::{
    Router,
    routing::{delete, get, post, put},
};

use crate::handlers::users::{
    create_user_handler, delete_user_handler, detail_user_handler, list_user_handler,
    update_user_handler,
};

pub fn user_routes() -> Router {
    Router::new()
        .route("/list", get(list_user_handler))
        .route("/detail/{id}", get(detail_user_handler))
        .route("/create", post(create_user_handler))
        .route("/update/{id}", put(update_user_handler))
        .route("/delete/{id}", delete(delete_user_handler))
}
