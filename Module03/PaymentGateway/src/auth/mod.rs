use serde::{Deserialize, Serialize};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

// Secret key for encoding and decoding
const SECRET_KEY: &[u8] = b"BLOODvIPER";

// Create JWT Token
pub fn create_token(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let expire = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() + 60 * 60; // 1 HR EXPIRATION

    let my_claims = Claims {
        sub: user_id.to_owned(),
        exp: expire as usize,
    };

    encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret(SECRET_KEY)
    )
}

// Decode JWT Token
pub fn decode_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let validation = Validation::new(Algorithm::HS256);
    decode::<Claims>(token, &DecodingKey::from_secret(SECRET_KEY), &validation)
        .map(|token_data| token_data.claims)
}