use actix_web::{web, App, HttpServer};
mod api;


#[actix_web::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(|| {
        App::new()
            .configure(api::config)
    })
        .bind("127.0.0.1:5500")?
        .run()
        .await
        .unwrap();
    println!("Gateway is fukn running !!");
}
