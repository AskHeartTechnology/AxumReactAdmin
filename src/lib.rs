pub mod common;
pub mod config;
pub mod handlers;
pub mod models;
pub mod openapi;
pub mod routes;
pub mod services;

use crate::{
    config::AppConfig,
    openapi::ApiDoc,
    routes::{api_routes, static_handler},
};
use axum::{Router, routing::get};
use tokio::net::TcpListener;
use utoipa::OpenApi;
use utoipa_swagger_ui::{BasicAuth, Config, SwaggerUi};

async fn health() -> &'static str {
    "Server is Healthy!"
}

pub fn build_router(config: AppConfig) -> Router {
    let api = Router::new().merge(api_routes());
    Router::new()
        .merge(
            SwaggerUi::new("/swagger-ui")
                .url("/api-docs/openapi.json", ApiDoc::openapi())
                .config(
                    Config::from("/api-docs/openapi.json").basic_auth(BasicAuth {
                        username: config.swagger.username,
                        password: config.swagger.password,
                    }),
                ),
        )
        .route("/health", get(health))
        .nest("/api", api)
        .fallback(static_handler)
}

pub async fn run() -> Result<(), std::io::Error> {
    let config = config::AppConfig::load().expect("项目配置加载失败");
    let addr = config.bind_addr();
    let listener = TcpListener::bind(addr).await?;
    println!("Server is listening on {} .", listener.local_addr()?);
    axum::serve(listener, build_router(config)).await
}
