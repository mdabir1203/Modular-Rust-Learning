// Setting up Warp filters
use warp::Filter;
use crate::handlers::{login, protected};
use crate::models::User;


pub fn login_filter() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("login")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(login)
}

pub fn validate_filter() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("protected")
        .and(warp::get())
        .and(warp::query::<User>())
        .and_then(protected)
}