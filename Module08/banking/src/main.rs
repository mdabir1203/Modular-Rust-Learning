use banking::config::get_config;
use banking::routes::create_routes;

#[tokio::main]
async fn main() {
    let config = get_config().await;
    let pool = config.create_db_pool().await.unwrap();
    let app = create_routes(pool);
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    dotenv().ok();  // Load environment variables from .env file
    
    axum::Server::bind(&config.server_address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}