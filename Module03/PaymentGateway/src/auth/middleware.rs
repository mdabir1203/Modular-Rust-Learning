use actix_web::{dev::ServiceRequest, Error, HttpMessage, HttpResponse, http::header};
use actix_service::{Service, Transform};
use futures_util::future::{ok, Ready};
use std::task::{Context, Poll};
use std::pin::Pin;
use futures_util::FutureExt;
use log::warn;
use serde_json::json;


// ... (include the AuthError enum and validate_token function here)
enum AuthError {
    MissingToken,
    InvalidToken,
    ExpiredToken,
    // Add other specific error types as needed
}

pub fn validate_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let validation = Validation {
        leeway: 60,
        ..Validation::new(Algorithm::HS512)
    };
pub struct AuthMiddleware {
    config: AuthMiddlewareConfig,
}

impl AuthMiddleware {
    pub fn new(config: AuthMiddlewareConfig) -> Self {
        Self { config }
    }
}

// ... (Transform implementation remains largely the same)

impl<S, B> Service<ServiceRequest> for AuthMiddlewareMiddleware<S>
where
    S: Service<ServiceRequest, Response = actix_web::dev::ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn futures_util::Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        if self.config.exempt_paths.contains(&req.path().to_string()) {
            return Box::pin(self.service.call(req));
        }
// Used and_then for avoiding unnecessary cloning of the token
        let token = req.headers()
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .and_then(|h| h.strip_prefix("Bearer ").map(|t| t.trim()));

        let auth_result = match token {
            Some(t) => validate_token(t),
            None => Err(AuthError::MissingToken),
        };

        match auth_result {
            Ok(()) => {
                Box::pin(self.service.call(req).map(|result| {
                    result.map(|mut res| {
                        res.response_mut().headers_mut().extend(add_security_headers(HttpResponse::Ok()).headers().clone());
                        res
                    })
                }))
            }
            Err(auth_error) => {
                warn!("Authentication failed: {:?}", auth_error);
                let error_response = HttpResponse::Unauthorized()
                    .json(json!({
                        "error": "Authentication failed",
                        "details": auth_error.to_string()
                    }));
                Box::pin(async move { Ok(req.into_response(error_response.into_body())) })
            }
        }
    }
}

fn add_security_headers(mut response: HttpResponse) -> HttpResponse {
    response
        .headers_mut()
        .insert(header::X_XSS_PROTECTION, "1; mode=block".parse().unwrap());
    response
        .headers_mut()
        .insert(header::X_FRAME_OPTIONS, "DENY".parse().unwrap());
    response
        .headers_mut()
        .insert(header::X_CONTENT_TYPE_OPTIONS, "nosniff".parse().unwrap());
    response
}