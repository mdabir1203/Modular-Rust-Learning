use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde_json::json;
mod models;

async fn get_user() -> impl Responder {
    HttpResponse::Ok().json(models::User {
        id: "1".to_string(),
        name: "John Doe".to_string(),
        email: "john.doe@example.com".to_string(),
    })
}

async fn delete_user(info: web::Path<(String,)>) -> impl Responder {
    // Logic for deleting user by ID
    HttpResponse::Ok().json(json!({"status": "user deleted"}))
}

async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/user", web::get().to(get_user))
            .route("/user/{id}", web::delete().to(delete_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/user", web::get().to(get_user))
            .route("/user/{id}", web::delete().to(delete_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await#[actix_web::main]
    async fn main() -> std::io::Result<()> {
        HttpServer::new(|| {
            App::new()
                .route("/user", web::get().to(get_user))
                .route("/user/{id}", web::delete().to(delete_user))
        })
        .bind("127.0.0.1:8080")?
        .run()
        .await
    }
}