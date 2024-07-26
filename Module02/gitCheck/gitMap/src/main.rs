use actix_web::{web, App, HttpServer};
mod handlers;
mod model;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/repo/{owner}/{repo}", web::get().to(handlers::get_repo_info))
    })
        .bind("127.0.0.1:5500")?
        .run()
        .await
}
