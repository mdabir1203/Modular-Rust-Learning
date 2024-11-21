use crate::db::account;
use crate::errors::AppError;
use sqlx::PgPool;

pub async fn create_account_service(pool: &PgPool, name: &str) -> Result<Account, AppError> {
    account::create_account(pool, name).await
}