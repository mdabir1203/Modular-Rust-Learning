use crate::db::account;
use crate::config::Config;
use sqlx::PgPool;

#[tokio::test]
async fn test_create_account() {
    let config = Config::from_env();
    let pool = PgPool::connect(&config.database_url).await.unwrap();

    let result = account::create_account(&pool, "Test Account").await;
    assert!(result.is_ok());
}