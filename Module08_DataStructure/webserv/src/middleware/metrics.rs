use axum::{
    middleware::Next,
    response::Response,
};
use metrics::{counter, histogram};
use std::time::Instant;

pub async fn track_metrics(
    req: axum::http::Request<axum::body::Body>,
    next: Next<axum::body::Body>,
) -> Response {
    let start = Instant::now();
    let path = req.uri().path().to_owned();
    let method = req.method().clone();

    let response = next.run(req).await;

    let duration = start.elapsed().as_secs_f64();
    histogram!(
        "http_request_duration_seconds",
        duration,
        "path" => path,
        "method" => method.to_string(),
    );
    counter!("http_requests_total", 1, "path" => path);

    response
}