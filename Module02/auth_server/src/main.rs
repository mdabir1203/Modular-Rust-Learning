mod handlers;
mod jwt;

use warp::{Filter};
use crate::handlers::{protected, login};



/// @brief The main function that sets up and runs the Warp server.
/// 
/// This function is the entry point of the application. It defines the routes for login and protected endpoints,
/// combines them, and starts the Warp server to listen for incoming requests.
/// 
/// @details
/// - The `login_route` handles POST requests to the `/login` path. It expects a JSON body and calls the `login` handler.
/// - The `protected_route` handles GET requests to the `/protected` path. It expects an `authorization` header and calls the `protected` handler.
/// - The routes are combined using the `or` method, allowing the server to handle both routes.
/// - The server listens on `127.0.0.1:8080`.
/// 
/// @return This function does not return anything as it runs the server indefinitely.
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
    warp::serve(routes).run(([127, 0, 0, 1], 5500)).await;
}