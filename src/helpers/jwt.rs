use crate::domain::auth::Claims;
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey, TokenData, errors::Result};
use std::env;

/// Genera un token JWT
pub fn generate_jwt(sub: String, role: String) -> Result<String> {
    let secret: String = env::var("JWT_SECRET").expect("JWT_SECRET debe estar configurado");
    let claims: Claims = Claims::new(sub, role, 60); 

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

/// Valida y decodifica un token JWT
pub fn validate_jwt(token: &str) -> Result<TokenData<Claims>> {
    let secret: String = env::var("JWT_SECRET").expect("JWT_SECRET debe estar configurado");

    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
}