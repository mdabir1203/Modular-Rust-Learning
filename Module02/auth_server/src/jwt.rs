use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};

// Define the Claims struct for JWT
#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}


// Generate a JWT token
pub fn generate_jwt(username: &str) -> String {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::seconds(60))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: username.to_owned(),
        exp: expiration as usize,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret("secret".as_ref())).unwrap()
}

// Validate the JWT token
pub fn jwt_validation(token: &str) -> bool {
    let validation = Validation::new(Algorithm::HS256);
    let token_data = decode::<Claims>(token, &DecodingKey::from_secret("secret".as_ref()), &validation);
    token_data.is_ok()
}
