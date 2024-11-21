use serde::{Deserialize, Serialize};
use bigdecimal::BigDecimal;
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Transaction {
    pub id: i32,
    pub from_account_id: Option<i32>,
    pub to_account_id: Option<i32>,
    pub amount: BigDecimal,
    pub transaction_type: String,
    pub created_at: DateTime<Utc>,
}