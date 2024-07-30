// Setting up a warp server and define routes for login + protected resources

use warp::http::StatusCode;
use warp::Filter;
use warp::reject;
use warp::reply::json;
use warp::{Rejection, Reply};
use serde_json::json;
use crate::models::User;
use crate::jwt::{generate_jwt, jwt_validation};

// Authenticating a user and returning a JWT if successful
pub async fn login(user: User) -> Result<impl Reply, Rejection> {
    if user.username == "admin" && user.password == "password" {
        let token = generate_jwt(&user.username);
        Ok(warp::reply::json(&json!({ "token": token })))
    } else {
        Ok(warp::reply::with_status(
            "Unauthorized",
            StatusCode::UNAUTHORIZED,
        ))
    }
}

// Checking the validity of JWT and grant access if valid
pub async fn protected(token: String) -> Result<impl Reply, Rejection> {
    if jwt_validation(&token) {
        Ok(warp::reply::with_status("Access granted", StatusCode::OK))
    } else {
        Ok(warp::reply::with_status(
            "Forbidden",
            StatusCode::FORBIDDEN,
        ))
    }
}


pub async fn unauthorized() -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::with_status(
        warp::reply::json(&"Unauthorized"),
        StatusCode::UNAUTHORIZED,
    ))
}