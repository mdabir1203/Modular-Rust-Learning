use actix_web::{web, HttpResponse, Responder};
use crate::models::user::{CreateUser, LoginUser};
use crate::services::auth;
use crate::errors::AppError;

pub async fn register(user: web::Json<CreateUser>, pool: web::Data<sqlx::PgPool>) -> Result<impl Responder, AppError> {
    let user = auth::register_user(user.into_inner(), &pool).await?;
    Ok(HttpResponse::Ok().json(user))
}

pub async fn login(creds: web::Json<LoginUser>, pool: web::Data<sqlx::PgPool>) -> Result<impl Responder, AppError> {
    let token = auth::login_user(creds.into_inner(), &pool).await?;
    Ok(HttpResponse::Ok().json(json!({ "token": token })))
}