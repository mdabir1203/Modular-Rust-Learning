use actix_web::{web, HttpResponse, Responder, dev::ServiceRequest, Error};
use actix_web::extractors::bearer::BearerAuth;
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use crate::model::Credentials;
use actix_web::error::ErrorUnauthorized;


pub async fn login(credentials: web::Json<Credentials>) -> impl Responder {
    if credentials.user == "admin" && credentials.pass == "admin" {
        match encode(&Header::default(), &credentials.user, &EncodingKey::from_secret("sec".as_ref())) {
            Ok(token) => HttpResponse::Ok().json(token),
            Err(_) => HttpResponse::InternalServerError().body("Token creation error"),
        }
    } else {
        HttpResponse::Unauthorized().body("Invalid Credentials")
    }
}

pub async fn auth_middleware(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, Error> {
    let token = credentials.token();
    let validation = Validation::default();
    let result = decode::<String>(
        &token,
        &DecodingKey::from_secret("sec".as_ref()),
        &validation,
    );

    match result {
        Ok(_) => Ok(req),
        Err(_) => Err(ErrorUnauthorized("Invalid Token")),
    }
}