mod auth;
mod errors;
mod routes;
mod jwt;
mod model;

use std::env;
use warp::Filter;
use tokio::signal;

#[tokio::main]
async fn main() {
    dotenv().ok();

    // Load configuration from environment variables
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port: u16 = env::var("PORT").unwrap_or_else(|_| "3030".to_string()).parse().expect("PORT must be a number");

    let routes = routes::routes();

    // Log server startup
    println!("Starting server at {}:{}", host, port);

    // Run the server with graceful shutdown
    let (_, server) = warp::serve(routes)
        .bind_with_graceful_shutdown(([host.parse().unwrap(), port]), async {
            signal::ctrl_c().await.expect("Failed to listen for ctrl_c signal");
        });

    server.await;
}