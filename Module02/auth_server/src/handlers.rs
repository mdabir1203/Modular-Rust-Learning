use warp::{Rejection, Reply};
use serde::{Deserialize};
use serde_json::json;
use crate::jwt::{generate_jwt, jwt_validation};



// Define the User struct
#[derive(Deserialize)]
pub struct User {
    username: String,
    password: String,
}


// Handle login requests
pub async fn login(user: User) -> Result<impl Reply, Rejection> {
    if user.username == "DragonBall" && user.password == "Vegeta" {
        let token = generate_jwt(&user.username);
        Ok(warp::reply::json(&json!({ "token": token })))
    } else {
        Ok(warp::reply::json(&json!("\nUnauthorized")))
    }
}

// Handle protected route requests
pub async fn protected(token: String) -> Result<impl Reply, Rejection> {
    if jwt_validation(&token) {
        Ok(warp::reply::json(&json!("\nAccess Granted")))
    } else {
        Ok(warp::reply::json(&json!("\nForbidden")))
    }
}
