use std::alloc::System;
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use crate::models::Claims;
use std::time::{SystemTime, UNIX_EPOCH};


const SECRET: &[u8] = b"Enter the Secret Key";

// Generating a JWT Token for a given username and expiration date of an hour
pub fn generate_jwt(username: &str) -> String {
    let exp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize
        * 60 * 60; // Token validity : 1 Hr

    let claims = Claims {
        sub: username.to_owned(),
        exp,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET)).unwrap()
}

// Validating a given JWT and return true if valid otherwise false
pub fn jwt_validation(token: &str) -> bool {
    decode::<Claims>(token, &DecodingKey::from_secret(SECRET), &Validation::default()).is_ok()
}