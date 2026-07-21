pub mod common;
pub mod config;
pub mod handlers;
pub mod models;
pub mod routes;
pub mod services;

use axum::{Router, http::StatusCode, response::IntoResponse, routing::get};
use tokio::net::TcpListener;

use crate::routes::api_routes;

async fn not_found() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "not_found")
}

async fn health() -> &'static str {
    "Server is Healthy!"
}

pub fn build_router() -> Router {
    let api = Router::new().merge(api_routes());
    Router::new()
        .route("/", get(health))
        .route("/health", get(health))
        .nest("/api", api)
        .fallback(not_found)
}

pub async fn run() -> Result<(), std::io::Error> {
    let config = config::AppConfig::load().unwrap();
    let addr = config.bind_addr();
    let listener = TcpListener::bind(addr).await?;
    println!("Server is listening on {} .", listener.local_addr()?);
    axum::serve(listener, build_router()).await
}
