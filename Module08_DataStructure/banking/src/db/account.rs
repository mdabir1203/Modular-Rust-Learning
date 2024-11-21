use crate::models::Account;
use crate::errors::AppError;
use sqlx::{PgPool, query_as};
use bigdecimal::BigDecimal;

pub async fn create_account(pool: &PgPool, name: &str) -> Result<Account, AppError> {
    query_as!(
        Account,
        "INSERT INTO accounts (name, balance) VALUES ($1, 0.0) RETURNING id, name, balance",
        name
    )
    .fetch_one(pool)
    .await
    .map_err(AppError::DatabaseError)
}