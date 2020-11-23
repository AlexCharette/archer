use alcoholic_jwt::{JWKS, Validation, ValidationError, token_kid, validate};
use bcrypt::{hash, verify, BcryptError, DEFAULT_COST};
use std::error::Error;

pub fn hash_password(password: String) -> Result<String, BcryptError> {
    hash(password, DEFAULT_COST)
}

pub fn verify_password(password: String, hash: String) -> Result<bool, BcryptError> {
    verify(password, &hash)
}

pub async fn validate_token(token: &str) -> Result<bool, ValidationError> {
    let authority = std::env::var("AUTHORITY").expect("AUTHORITY must be set");
    let jwks = fetch_jwks(&format!(
        "{}{}",
        authority.as_str(),
        ".well-known/jwks.json"
    ))
    .await
    .expect("Failed to fetch jwks");
    let validations = vec![Validation::Issuer(authority), Validation::SubjectPresent];
    let kid = match token_kid(&token) {
        Ok(res) => res.expect("Failed to decode kid"),
        Err(err) => {
            return Err(err);
        },
    };
    let jwk = jwks.find(&kid).expect("Specified key not found in set");
    let res = validate(token, jwk, validations);
    Ok(res.is_ok())
}

async fn fetch_jwks(uri: &str) -> Result<JWKS, Box<dyn Error>> {
    let res = reqwest::get(uri).await?;
    let val = res.json::<JWKS>().await?;
    return Ok(val);
}
