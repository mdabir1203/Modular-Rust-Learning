// src/main.rs

use actix_web::{web, App, HttpServer, HttpResponse, HttpMessage};
use crate::auth::{create_token, AuthMiddleware};

mod auth;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(AuthMiddleware)
            .route("/login", web::post().to(login))
            .route("/secure", web::get().to(secure))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

async fn login() -> HttpResponse {
    let user_id = "user123";
    let token = create_token(user_id);
    HttpResponse::Ok().body(token)
}

async fn secure(req: actix_web::HttpRequest) -> HttpResponse {
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") {
                let token = auth_str.trim_start_matches("Bearer ");
                if let Ok(claims) = validate_token(token) {
                    return HttpResponse::Ok().json(claims);
                }
            }
        }
    }
    HttpResponse::Unauthorized().finish()
}