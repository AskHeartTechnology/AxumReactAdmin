pub mod users;

use axum::{
    Router,
    http::{StatusCode, Uri, header},
    response::{Html, IntoResponse, Response},
};
use rust_embed::Embed;

static INDEX_HTML: &str = "index.html";

#[derive(Embed)]
#[folder = "app/dist/"]
struct Assets;

/// Axum fallback：托管嵌入的前端资源（含 SPA 路由回退）
pub async fn static_handler(uri: Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/');

    if path.is_empty() || path == INDEX_HTML {
        return index_html().await;
    }

    match Assets::get(path) {
        Some(content) => {
            let mime = mime_guess::from_path(path).first_or_octet_stream();
            ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
        }
        None => {
            // 带扩展名却找不到 → 真 404（如 /assets/xxx.js 缺失）
            if path.contains('.') {
                return not_found().await;
            }
            // 无扩展名 → 前端路由，回退 index.html
            index_html().await
        }
    }
}

async fn index_html() -> Response {
    match Assets::get(INDEX_HTML) {
        Some(content) => Html(content.data).into_response(),
        None => not_found().await,
    }
}

async fn not_found() -> Response {
    (StatusCode::NOT_FOUND, "pages not_found").into_response()
}

pub fn api_routes() -> Router {
    Router::new().nest("/users", users::user_routes())
}
