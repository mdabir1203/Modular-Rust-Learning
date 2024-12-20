use sqlx::{PgPool, Pool, Postgres};
use crate::config::Config;

pub async fn create_pool(config: &Config) -> PgPool {
    PgPool::connect(&config.database_url)
    .await
    .expect("Failed to create database pool")
}