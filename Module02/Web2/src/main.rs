use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[get("/")]
async fn intro() -> impl Responder {
    HttpResponse::Ok().body("<h1 style='color:green;'>Welcome to Rusty server</h1>")
}

#[get("/intro/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Welcome, my little peers {}!", name)
}

#[derive(Serialize, Deserialize)]
struct Info {
    username: String,
}

#[post("/echo")]
async fn echo(info: web::Json<Info>) -> impl Responder {
    HttpResponse::Ok().json(info.0)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server starts running .... ");
    HttpServer::new(|| App::new().service(intro).service(greet).service(echo))
        .bind("127.0.0.1:5500")?
        .run()
        .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http::StatusCode, test};

    #[actix_rt::test]
    async fn test_intro() {
        let mut app = test::init_service(App::new().service(intro)).await;

        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_greet() {
        let mut app = test::init_service(App::new().service(greet)).await;

        let req = test::TestRequest::get().uri("/intro/World").to_request();
        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_echo() {
        let mut app = test::init_service(App::new().service(echo)).await;

        let req = test::TestRequest::post()
            .uri("/echo")
            .set_json(&Info {
                username: "World".to_string(),
            })
            .to_request();
        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }
}
