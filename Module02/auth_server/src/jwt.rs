use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};

// Define the Claims struct for JWT
#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}


/// Generates a JSON Web Token (JWT) for a given username
///
/// This function creates a secure token that represents a user's identity and includes an expiration time.
/// It's like creating a temporary digital ID card for the user.
///
/// @param username The name or identifier of the user for whom the token is being generated
///
/// @return A string containing the JWT, which can be used for authentication in other parts of the system
///
/// @details
/// The function works as follows:
/// 1. It calculates an expiration time 60 seconds from now.
/// 2. It creates a Claims struct with the username and expiration time.
/// 3. It encodes this information into a JWT using a secret key.
///
/// The resulting JWT can be sent to the client and used for subsequent authenticated requests.
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

/// Validates a JSON Web Token (JWT)
///
/// This function checks if a given JWT is valid and hasn't expired.
/// It's like checking if an ID card is genuine and still current.
///
/// @param token The JWT string to be validated
///
/// @return A boolean value: true if the token is valid, false otherwise
///
/// @details
/// The function works as follows:
/// 1. It sets up the validation parameters, specifying the expected algorithm (HS512).
/// 2. It attempts to decode the token using the same secret key used for encoding.
/// 3. If the decoding is successful (i.e., the token is valid and not expired), it returns true.
///   Otherwise, it returns false.
///
/// This function can be used to verify tokens sent by clients before allowing access to protected resources.
pub fn jwt_validation(token: &str) -> bool {
    let validation = Validation::new(Algorithm::HS512);
    let token_data = decode::<Claims>(token, &DecodingKey::from_secret("secret".as_ref()), &validation);
    token_data.is_ok()
}
