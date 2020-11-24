use aes_gcm::aead::generic_array::GenericArray;
use aes_gcm::aead::{Aead, NewAead};
use aes_gcm::Aes256Gcm;
use alcoholic_jwt::{token_kid, validate, Validation, ValidationError, JWKS};
use bcrypt::{hash, verify, BcryptError, DEFAULT_COST};
// use rand::{thread_rng, Rng};
// use ring::aead::{Aad, AES_256_GCM, BoundKey, Nonce, NonceSequence, OpeningKey, SealingKey};
use std::error::Error;

use super::AES_KEY;

pub fn hash_password(password: String) -> Result<String, BcryptError> {
    hash(password, DEFAULT_COST)
}

pub fn verify_password(password: String, hash: String) -> Result<bool, BcryptError> {
    verify(password, &hash)
}

pub fn encrypt_private_key(public_key: String, private_key: String) -> String {
    let pt_bytes = private_key.as_bytes();
    let aes_key = AES_KEY.clone();
    let aes_key = aes_key.as_bytes();
    let key = GenericArray::from_slice(aes_key);
    let cipher = Aes256Gcm::new(key);
    // let mut nonce = thread_rng().gen::<[u8; 12]>();

    // thread_rng()
    //     .try_fill(&mut nonce[..])
    //     .expect("Error generating random nonce");

    let nonce = GenericArray::from_slice(public_key.as_bytes());

    let ciphertext = cipher
        .encrypt(&nonce, pt_bytes.as_ref())
        .expect("Could not encrypt text");
    String::from_utf8(ciphertext).expect("Could not convert bytes to string")
}

pub fn decrypt_private_key(public_key: String, private_key: String) -> String {
    let ct_bytes = private_key.as_bytes();
    let aes_key = AES_KEY.clone();
    let aes_key = aes_key.as_bytes();
    let key = GenericArray::from_slice(aes_key);
    let cipher = Aes256Gcm::new(key);
    // let mut nonce = thread_rng().gen::<[u8; 12]>();

    // thread_rng()
    //     .try_fill(&mut nonce[..])
    //     .expect("Error generating random nonce");

    let nonce = GenericArray::from_slice(public_key.as_bytes());

    let plaintext = cipher
        .decrypt(&nonce, ct_bytes.as_ref())
        .expect("Could not decrypt text");
    String::from_utf8(plaintext).expect("Could not convert bytes to string")
}

/*
// TODO is this necessary?
async def _authorize(self, request):
        token = request.headers.get('AUTHORIZATION')
        if token is None:
            raise ApiUnauthorized('No auth token provided')
        token_prefixes = ('Bearer', 'Token')
        for prefix in token_prefixes:
            if prefix in token:
                token = token.partition(prefix)[2].strip()
        try:
            token_dict = deserialize_auth_token(request.app['secret_key'],
                                                token)
        except BadSignature:
            raise ApiUnauthorized('Invalid auth token')
        public_key = token_dict.get('public_key')

        auth_resource = await self._database.fetch_auth_resource(public_key)
        if auth_resource is None:
            raise ApiUnauthorized('Token is not associated with an agent')
        return decrypt_private_key(request.app['aes_key'],
                                   public_key,
                                   auth_resource['encrypted_private_key'])
*/

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
        }
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
