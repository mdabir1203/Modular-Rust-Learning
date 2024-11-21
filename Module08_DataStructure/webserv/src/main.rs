use axum::{
    routing::get,
    Router,
};
use production_server::{
    config::AppConfig,
    middleware,
    routes,
};
use std::net::SocketAddr;
use tower_http::{
    cors::CorsLayer,
    trace::TraceLayer,
};
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load environment variables
    dotenv::dotenv().ok();

    // Initialize logging
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    // Load configuration
    let config = AppConfig::new()?;

    // Build application
    let app = Router::new()
        .route("/health", get(routes::health::health_check))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .layer(axum::middleware::from_fn(middleware::metrics::track_metrics));

    // Start server
    let addr = SocketAddr::new(
        config.server.host.parse()?,
        config.server.port,
    );

    info!("Starting server on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}