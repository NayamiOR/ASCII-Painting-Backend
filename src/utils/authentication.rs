use axum::http::HeaderMap;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::error::Error;

// todo: 换掉key
const SECRET_KEY: &[u8] = b"your_secret_key";

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Claims {
    pub(crate) sub: String,
    pub(crate) id: i32,
    pub(crate) exp: usize,
}

pub(crate) fn generate_jwt(email: &str, id: i32) -> String {
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
        + 60 * 60; // 1 hour expiration

    let claims = Claims {
        sub: email.to_owned(),
        id,
        exp: expiration as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET_KEY),
    )
    .unwrap()
}

pub(crate) fn generate_refresh_token() -> String {
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};

    let random_token: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32) // 32 字符的随机字符串
        .map(char::from)
        .collect();

    random_token
}

pub(crate) fn validate_jwt(token: &str) -> bool {
    if decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET_KEY),
        &Validation::default(),
    )
    .is_err()
    {
        return false;
    }
    // check expiration
    let claims = extract_claims(token).unwrap();
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as usize;
    if now > claims.exp {
        return false;
    }
    true
}

pub(crate) fn extract_claims(token: &str) -> Result<Claims, Error> {
    match decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET_KEY),
        &Validation::default(),
    ) {
        Ok(token_data) => Ok(token_data.claims),
        Err(_) => Err(Error::Server(Box::new(
            crate::error::ServerError::Unauthorized,
        ))),
    }
}

pub(crate) fn extract_claims_from_header(header_map: HeaderMap) -> Result<Claims, Error> {
    // check Authorization header
    if !header_map.contains_key("Authorization") {
        return Err(Error::Server(Box::new(
            crate::error::ServerError::Unauthorized,
        )));
    }
    let header = header_map.get("Authorization").unwrap().to_str().unwrap();
    let token = header.split_whitespace().last().unwrap();
    extract_claims(token)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test::get_random_string;

    #[test]
    fn test_jwt() {
        // test generate_jwt and validate_jwt
        let name = get_random_string(16);
        let email = format!("{}@example.com", name);
        let id = 1;
        let jwt = generate_jwt(email.as_str(), id);
        assert!(validate_jwt(jwt.as_str()));
        dbg!(&jwt);

        // test extract_jwt
        let claims = extract_claims(jwt.as_str()).unwrap();
        assert_eq!(claims.sub, email);
        assert_eq!(claims.id, id);
    }
}
