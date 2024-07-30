mod routes;
use crate::routes::{login_filter, validate_filter};
use warp::Filter;

#[tokio::main]
async fn main() {
    let login = routes::login_filter();
    let validate = routes::validate_filter();

    let routes = (login).or(validate);

    warp::serve(routes)
        .run(([127,0,0,1], 5500))
        .await;
    println!("Server is running with JWT!");
}
