use warp::{Rejection, Reply, reject};
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
pub async fn login(user: User) -> Result<impl Reply, CustomError> {
    match (user.username.as_str(), user.password.as_str()) {
        ("DragonBall", "Vegeta") => {
            let token = generate_jwt(&user.username);
            Ok(warp::reply::json(&json!({ "token": token })))
        },
        _ => Ok(warp::reply::json(&json!("\nInvalid credentials"))),
    }
}

// Handle protected route requests
pub async fn protected(token: String) -> Result<impl Reply, CustomError> {
    if jwt_validation(&token) {
        Ok(warp::reply::json(&json!("\nProtected route accessed")))
    } else {
        Ok(warp::reply::json(&json!("\nInvalid token")))
    }
}

// Define your custom error type
#[derive(Debug)]
pub enum CustomError {
    InvalidToken,
}

impl reject::CombineRejection<Rejection> for CustomError {}

#[cfg(test)]
mod tests {
    use super::*;
    use warp::http::StatusCode;
    use warp::test::{request, Response};

    #[tokio::test]
    async fn test_login() {
        let user = User {
            username: "DragonBall".to_string(),
            password: "Vegeta".to_string(),
        };

        let res = login(user).await.unwrap();
        assert_eq!(Response::status(&res), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_protected() {
        let token = generate_jwt("DragonBall");
        let res = protected(token).await.unwrap();
        assert_eq!(Response::status(&res), StatusCode::OK);
    }
}