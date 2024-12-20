use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::Error;
use actix_web::error::ErrorUnauthorized;
use actix_web::http::header::{HeaderName, HeaderValue};
use actix_web::web::Data;
use futures::future::{ok, Ready};
use jsonwebtoken::{decode, DecodingKey, Validation};

use crate::config::Settings;

pub struct Authentication;

impl<S, B> Transform<S> for Authentication
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthenticationMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthenticationMiddleware { service })
    }
}

pub struct AuthenticationMiddleware<S> {
    service: S,
}

impl<S, B> Service for AuthenticationMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let auth_header = req.headers().get("Authorization");
        let config = req.app_data::<Data<Settings>>().unwrap();

        match auth_header {
            Some(auth_header) => {
                let auth_token = auth_header.to_str().unwrap().split("Bearer ").collect::<Vec<&str>>()[1];
                let token_data = match decode::<Claims>(
                    &auth_token,
                    &DecodingKey::from_secret(config.jwt_secret.as_ref()),
                    &Validation::default(),
                ) {
                    Ok(c) => c,
                    Err(_) => return Box::pin(async move { Err(ErrorUnauthorized("Invalid token")) }),
                };

                req.extensions_mut().insert(token_data.claims);

                let fut = self.service.call(req);
                Box::pin(async move {
                    let res = fut.await?;
                    Ok(res)
                })
            }
            None => Box::pin(async move { Err(ErrorUnauthorized("Authorization header missing")) }),
        }
    }
}