use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/")] // route handler for root
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
}
