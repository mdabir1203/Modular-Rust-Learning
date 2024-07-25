// TODO : Help me add POST method
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use serde::Serialize;

#[get("/")]   // route handler for root
async fn intro() -> impl Responder {
    HttpResponse::Ok().body("<h1 style='color:green;'>Welcome to Rusty server</h1>")
}

#[get("/intro/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder
{
    format!("Welcome,my little peers {}!", name)
}

#[derive(Serialize,Deserialize)]
struct Info {
    username: String,
}

#[post("/echo")]
async fn echo(info: web::Json<Info>) -> impl Responder
{
    HttpResponse::Ok().json(info.0)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {App::new()
        .service(intro)
        .service(greet)
        .service(echo)})
        .bind("127.0.0.1:5500")?
        .run()
        .await

}