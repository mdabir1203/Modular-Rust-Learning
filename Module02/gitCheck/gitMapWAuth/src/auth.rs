use actix_web::{web, HttpResponse, Responder};
use reqwest;
use serde_json::from_str;
use crate::model::RepoInfo;
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};


pub async fn login(credentials: web::Json<Credentials>) -> impl Responder {
    // check for valid credential
    if credentials.user == "admin" && credentials.pass == "admin" {
        // create token
        let token = encode(&Header::default(), &credentials.user, &EncodingKey::from_secret("sec".as_ref()),
   ).unwrap();
        // Return the token as the response
        HttpResponse::Ok().json(token)
        } else {
        // Return an error response if creds are invalid
        HttpResponse::Unauthorized().body("Invalid Credentials")
        }
}

// Implementing middleware
async fn auth_middleware(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, actix_web::Error> {
    // Validate the JWT token
    let token = credentials.token();
    let validation = Validation::default();
    let result = decode::<String> (
        &token,
        &DecodingKey::from_secret("sec".as_ref()),
        &validation,
    );

    // Check token validity
    match result {
        Ok(_) => Ok(req),
        Err(_) => Err(actix_web::ErrorUnauthorized("Invalid Token")),
    }
}