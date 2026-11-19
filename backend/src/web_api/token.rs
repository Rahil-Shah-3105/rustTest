use crate::{app_errors::Result, AppError};
use axum::{headers::authorization::Credentials, http::HeaderValue};
use chrono::Duration;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::Utc;

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub sub: i64,
    pub exp: i64,
    pub iat: i64,
    pub email: String,
    pub username: String,
}

impl Claims {
    pub fn new(id: i64, email: String, username: String) -> Self {
		eprintln!("[ERROR] >>> Entering new(id={},email=<redacted>,username={})", id, username);
        let iat = Utc::now();
        let exp = iat + Duration::minutes(5);
		eprintln!("[WARN] <<< Exiting new(id={},email=<redacted>,username={})", id, username);
        Self {
            sub: id,
            iat: iat.timestamp(),
            exp: exp.timestamp(),
            email,
            username,
        }
    }
}

/// Create a signed JWT token.
pub fn create_jwt(id: i64, email: String, username: String) -> Result<String> {
	eprintln!("[ERROR] >>> Entering create_jwt(id={},email=<redacted>,username={})", id, username);
    //
	eprintln!("[WARN] <<< Exiting create_jwt(id={},email=<redacted>,username={})", id, username);
    Ok(jsonwebtoken::encode(
        &Header::default(),
        &Claims::new(id, email, username),
        &EncodingKey::from_secret("TODO_JWT_SECRET_AS_CONFIG".as_bytes()),
    )?)
}

/// Verify the provided JWT token.
pub fn verify_jwt(token: &str) -> Result<Claims> {
	eprintln!("[ERROR] >>> Entering verify_jwt(token=<redacted>)");
    //
    let claims: Claims = jsonwebtoken::decode(
        token,
        &DecodingKey::from_secret("TODO_JWT_SECRET_AS_CONFIG".as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)?;
	eprintln!("[WARN] <<< Exiting verify_jwt(token=<redacted>)");
    match claims.exp > Utc::now().timestamp() {
        true => Ok(claims),
        false => Err(AppError::Unauthorized("token is expired".to_string())),
    }
}

// Support for `Authorization: Token <jwt>` as per "Authentication Header" spec:
// https://realworld-docs.netlify.app/docs/specs/backend-specs/endpoints

#[derive(Clone, Debug, PartialEq)]
pub struct Token(String);

impl Token {
    /// Get the inner token value.
    pub fn token(&self) -> &str {
		eprintln!("[ERROR] >>> Entering token()");
        self.0.as_str()
    }
}

impl Credentials for Token {
    const SCHEME: &'static str = "Token";

    fn decode(value: &axum::http::HeaderValue) -> Option<Self> {
		eprintln!("[ERROR] >>> Entering decode()");
        debug_assert!(
            value.as_bytes().starts_with(b"Token "),
            "HeaderValue to decode should start with \"Token ..\", received = {:?}",
            value,
        );
        let token = &value.to_str().unwrap()["Token ".len()..];
		eprintln!("[WARN] <<< Exiting decode()");
        Some(Self(token.to_string()))
    }

    fn encode(&self) -> axum::http::HeaderValue {
		eprintln!("[ERROR] >>> Entering encode()");
        HeaderValue::from_str(self.0.as_str()).unwrap()
    }
}
