use actix_web::{web, App, HttpServer};
use sqlx::postgres::PgPoolOptions;
use dotenv::dotenv;


mod config;
mod errors;
mod models;
mod handlers;
mod middleware;
mod db;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let config = config::load_config().expect("Failed to load configuration");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .expect("Failed to create pool");

    log::info!("Starting server at http://{}:{}", config.host, config.port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::auth::Authentication)
            .service(
                web::scope("/api")
                .service(handlers::health::health_check)
                .service(handlers::auth::login)
                .service(handlers::auth::register)
            )
    })
    .bind(format!("{}:{}", config.host, config.port))?
    .run()
    .await
}