use axum::{
    extract::State,
    response::IntoResponse,
    Json,
};
use serde::Serialize;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize)]
pub struct HealthResponse {
    status: String,
    timestamp: u64,
    version: String,
}

pub async fn health_check() -> impl IntoResponse {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    Json(HealthResponse {
        status: "healthy".to_string(),
        timestamp,
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}