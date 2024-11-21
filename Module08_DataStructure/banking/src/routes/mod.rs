use axum::{Router, routing::post};
use crate::db::DbPool;
use std::sync::Arc;

pub fn create_routes(pool: Arc<DbPool>) -> Router {
    Router::new()
        .route("/accounts", post(create_account_handler))
        .route("/accounts/:id/deposit", post(deposit_handler))
        .layer(axum::extract::Extension(pool))
}