// src/models/account.rs
use serde::{Deserialize, Serialize};
use bigdecimal::BigDecimal;
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Account {
    pub id: i32,
    pub name: String,
    pub balance: BigDecimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewAccount {
    pub name: String,
}
