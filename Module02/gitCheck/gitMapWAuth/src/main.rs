use actix_web::{web, App, HttpServer};
use actix_web::middleware::Logger;
mod handlers;
mod model;
mod auth;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(auth::auth_middleware)
            .service(handlers::intro)
            .service(handlers::greet)
            .service(handlers::echo)
            .route("/repo/{owner}/{repo}", web::get().to(handlers::get_repo_info))
    })
    .bind("127.0.0.1:5500")?
    .run()
    .await
}