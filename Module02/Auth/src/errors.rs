use warp::{http::StatusCode, reject, Rejection, Reply};
use std::convert::Infallible;
use thiserror::Error;
use log::error;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Invalid token")]
    InvalidToken,
    #[error("Token has expired")]
    ExpiredToken,
}

impl reject::Reject for AuthError {}

#[derive(Debug)]
pub enum CustomError {
    NotFound,
    Unauthorized,
    InternalError,
}

impl reject::Reject for CustomError {}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    if let Some(auth_error) = err.find::<AuthError>() {
        let (code, message) = match auth_error {
            AuthError::InvalidCredentials => (StatusCode::UNAUTHORIZED, "Invalid credentials"),
            AuthError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid token"),
            AuthError::ExpiredToken => (StatusCode::UNAUTHORIZED, "Token has expired"),
        };
        error!("Auth error occurred: {} - {}", code, message);
        return Ok(warp::reply::with_status(message, code));
    }

    if let Some(custom_error) = err.find::<CustomError>() {
        let (code, message) = match custom_error {
            CustomError::NotFound => (StatusCode::NOT_FOUND, "Not Found"),
            CustomError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized"),
            CustomError::InternalError => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error"),
        };
        error!("Custom error occurred: {} - {}", code, message);
        return Ok(warp::reply::with_status(message, code));
    }

    if err.is_not_found() {
        return Ok(warp::reply::with_status("Route not found", StatusCode::NOT_FOUND));
    }

    if let Some(_) = err.find::<warp::filters::body::BodyDeserializeError>() {
        return Ok(warp::reply::with_status("Invalid body", StatusCode::BAD_REQUEST));
    }

    if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
        return Ok(warp::reply::with_status("Method not allowed", StatusCode::METHOD_NOT_ALLOWED));
    }

    error!("Unhandled rejection: {:?}", err);
    Ok(warp::reply::with_status(
        "Unhandled rejection",
        StatusCode::INTERNAL_SERVER_ERROR,
    ))
}