use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/")]   // route handler for root
async fn intro() -> impl Responder {
    HttpResponse::Ok().body("Welcome to Rusty server")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(intro))
        .bind("127.0.0.1:8080")?
        .run()
        .await

}