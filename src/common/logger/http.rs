use axum::http::Request;
use tracing::Span;

pub fn request_span<B>(request: &Request<B>) -> Span {
    let request_id = request
        .headers()
        .get("x-request-id")
        .and_then(|value| value.to_str().ok())
        .unwrap_or("-");

    tracing::info_span!(
        "http_request",
        request_id = %request_id,
        method = %request.method(),
        path = %request.uri().path(),
    )
}
