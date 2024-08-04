use actix_web::{web, App, HttpServer};

mod handlers;
mod model;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::scope("/repo").route("/{owner}/{repo}", web::get().to(handlers::get_repo_info)))
    })
    .bind("127.0.0.1:5500")?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, http::StatusCode};

    #[actix_rt::test]
    async fn test_get_repo_info() {
        let mut app = test::init_service(
            App::new().service(web::scope("/repo").route("/{owner}/{repo}", web::get().to(handlers::get_repo_info)))
        ).await;

        let req = test::TestRequest::get().uri("/repo/rust-lang/rust").to_request();
        let resp = test::call_service(&mut app, req).await;
        
        assert_eq!(resp.status(), StatusCode::OK);
    }
}