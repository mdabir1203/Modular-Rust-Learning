use warp::{Filter, Rejection, Reply};
use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use serde_json::json;

// Define the User struct
#[derive(Deserialize)]
struct User {
    username: String,
    password: String,
}

// Define the Claims struct for JWT
#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

// Generate a JWT token
fn generate_jwt(username: &str) -> String {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::seconds(60))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: username.to_owned(),
        exp: expiration as usize,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret("secret".as_ref())).unwrap()
}

// Validate the JWT token
fn jwt_validation(token: &str) -> bool {
    let validation = Validation::new(Algorithm::HS256);
    let token_data = decode::<Claims>(token, &DecodingKey::from_secret("secret".as_ref()), &validation);
    token_data.is_ok()
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
        return Ok(StatusCode:: Access_Granted)
    } else {
        Ok(warp::reply::json(&json!("\nForbidden")))
    }
}

#[tokio::main]
async fn main() {
    // Define the login route
    let login_route = warp::post()
        .and(warp::path("login"))
        .and(warp::body::json())
        .and_then(login);

    // Define the protected route
    let protected_route = warp::get()
        .and(warp::path("protected"))
        .and(warp::header::<String>("authorization"))
        .and_then(|auth_header: String| {
            let token = auth_header.trim_start_matches("Bearer ").to_string();
            protected(token)
        });

    // Combine routes
    let routes = login_route.or(protected_route);

    // Start the Warp server
    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}