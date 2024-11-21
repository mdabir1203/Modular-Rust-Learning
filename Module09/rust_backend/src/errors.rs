use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Internal Server Error")]
    InternalServerError,
    #[error("Bad Request")]
    BadRequest,
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Not Found")]
    NotFound,
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::InternalServerError => {
                HttpResponse::InternalServerError().json("Internal Server Error")
            }
            AppError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
            AppError::Unauthorized => HttpResponse::Unauthorized().json("Unauthorized"),

        }
    }
}