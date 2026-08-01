pub mod common;
pub mod config;
pub mod handlers;
pub mod models;
pub mod openapi;
pub mod routes;
pub mod services;

use crate::{
    common::logger,
    common::logger::http::request_span,
    config::AppConfig,
    openapi::ApiDoc,
    routes::{api_routes, static_handler},
};
use axum::{Router, routing::get};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::{BasicAuth, Config, SwaggerUi};

async fn health() -> &'static str {
    "Server is Healthy!"
}

pub fn build_router(config: AppConfig) -> Router {
    let api = Router::new().merge(api_routes());

    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(request_span)
        .on_request(|request: &axum::http::Request<_>, _span: &tracing::Span| {
            log_info!(
                method = %request.method(),
                uri = %request.uri(),
                "HTTP 请求开始"
            );
        })
        .on_response(
            |response: &axum::http::Response<_>,
             latency: std::time::Duration,
             _span: &tracing::Span| {
                log_info!(
                    status = response.status().as_u16(),
                    latency_ms = latency.as_millis(),
                    "HTTP 请求完成"
                );
            },
        )
        .on_failure(
            |error: tower_http::classify::ServerErrorsFailureClass,
             latency: std::time::Duration,
             _span: &tracing::Span| {
                log_error!(
                    error = ?error,
                    latency_ms = latency.as_millis(),
                    "HTTP 请求失败"
                );
            },
        );

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
        .layer(trace_layer)
}

pub async fn run() -> Result<(), std::io::Error> {
    let config = config::AppConfig::load().expect("项目配置加载失败");
    let addr = config.bind_addr();
    logger::init(&config);
    let listener = TcpListener::bind(addr).await?;
    log_info!("Server is listening on {} .", listener.local_addr()?);
    axum::serve(listener, build_router(config)).await
}
