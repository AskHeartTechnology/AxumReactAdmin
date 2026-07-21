pub mod users;

use axum::Router;

pub fn api_routes() -> Router {
    Router::new().nest("/users", users::user_routes())
}
