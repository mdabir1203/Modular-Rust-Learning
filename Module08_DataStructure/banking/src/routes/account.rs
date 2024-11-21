use axum::{extract::State, Json};
use crate::{db::account, models::NewAccount, AppError};
use crate::db::create_account;
use crate::db::Account;

pub async fn create_account_handler(
    State(pool): State<PgPool>,
    Json(payload): Json<NewAccount>,
) -> Result<Json<Account>, AppError> {
    let account = account::create_account(&pool, &payload.name).await?;
    Ok(Json(account))
}