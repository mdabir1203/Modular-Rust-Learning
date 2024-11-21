use warp::Filter;
use warp::filter::FilterBase;
use crate::auth::{serve_form, authenticate, generate_token};
use crate::errors::handle_rejection;

pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let form_route = warp::path::end()
        .and_then(serve_form)
        .map_err(|err: std::convert::Infallible| warp::reject::custom(err));

    let auth_route = warp::path("authenticate")
        .and(warp::post())
        .and(warp::body::form())
        .and_then(authenticate)
        .map_err(|err: warp::Rejection| err);

    let token_route = warp::path("token")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(generate_token)
        .map_err(|err: warp::Rejection| err);

    form_route.or(auth_route).or(token_route).recover(handle_rejection)
}