use warp::Filter;
use warp::Reply;

mod handlers;
mod jwt;

use handlers::{login, protected, User};
use jwt::{generate_jwt, jwt_validation};

#[tokio::main]
async fn main() {
    // Define the protected route
    let protected_route = warp::get()
        .and(warp::path("protected"))
        .and(warp::header::<String>("authorization"))
        .and_then(protected);

    // Define the login route
    let login_route = warp::post()
        .and(warp::path("login"))
        .and(warp::body::json())
        .and_then(login);

    // Combine routes
    let routes = login_route.or(protected_route);

    // Start the Warp server
    warp::serve(routes).run(([127, 0, 0, 1], 5500)).await;
}

#[cfg(test)]
mod tests {
    use super::*;
    use warp::http::StatusCode;
    use warp::test::request;
    use warp::Reply;

    #[tokio::test]
    async fn test_login() {
        let user = User {
            username: "DragonBall".to_string(),
            password: "Vegeta".to_string(),
        };

        let res = login(user).await.unwrap();
        use warp::reply::json;

        let body = json(&res);
        assert_eq!(body.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_protected() {
        let token = generate_jwt("DragonBall");
        let res = protected(token).await.unwrap();
        let body = warp::test::Reply::json::<warp::reply::Json>(&res).await;
        assert_eq!(body.status(), StatusCode::OK);
    }
}
