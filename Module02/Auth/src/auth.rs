use warp::{http::StatusCode, reject, reply, Rejection, Reply};
use crate::model::User;
use crate::jwt::{generate_jwt, validate_jwt};
use crate::errors::{AuthError, CustomError};
use warp::reply::html;

use serde::Deserialize;
use log::error;
use std::env;
use std::convert::Infallible;

pub async fn login(user: User) -> Result<impl Reply, Rejection> {
    let expected_username = env::var("USERNAME").expect("USERNAME must be set");
    let expected_password = env::var("PASSWORD").expect("PASSWORD must be set");

    if user.username == expected_username && user.password == expected_password {
        let token = generate_jwt(&user.username);
        Ok(reply::json(&token))
    } else {
        Err(reject::custom(AuthError::InvalidCredentials))
    }
}

pub async fn protected(token: String) -> Result<impl Reply, Rejection> {
    match validate_jwt(&token) {
        Ok(_) => Ok(reply::with_status("Access granted", StatusCode::OK)),
        Err(_) => Err(reject::custom(AuthError::InvalidToken)),
    }
}

#[derive(Deserialize)]
struct AuthForm {
    token: String,
}

pub async fn serve_form() -> Result<impl Reply, Infallible> {
    let html_form = r#"
        <html>
            <body>
                <form action="/authenticate" method="post">
                    <label for="token">JWT Token:</label>
                    <input type="text" id="token" name="token">
                    <input type="submit" value="Submit">
                </form>
            </body>
        </html>
    "#;
    Ok(html(html_form))
}

pub async fn authenticate(form: AuthForm) -> Result<impl Reply, Rejection> {
    match validate_jwt(&form.token) {
        Ok(_) => Ok(warp::reply::with_status("Authentication successful", StatusCode::OK)),
        Err(_) => {
            error!("Authentication failed for token: {}", form.token);
            Err(reject::custom(CustomError::Unauthorized))
        },
    }
}

pub async fn generate_token(body: serde_json::Value) -> Result<impl warp::Reply, warp::Rejection> {
    let username = body.get("username").and_then(|v| v.as_str()).unwrap_or("");
    let token = generate_jwt(username);
    Ok(warp::reply::json(&token))
}