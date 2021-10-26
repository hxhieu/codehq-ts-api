extern crate alcoholic_jwt;

use alcoholic_jwt::JWKS;
use std::sync::RwLock;

lazy_static! {
    static ref KEY_STORE: RwLock<Option<JWKS>> = RwLock::new(None);
}

pub async fn get_jwks() -> Option<JWKS> {
    return KEY_STORE.read().unwrap().to_owned();
}

pub async fn validate_token(token: &str, key_store: &JWKS) -> Result<bool, String> {
    if token.len() == 0 {
        return Err("Bearer token is required".to_string());
    }
    // TODO: Actually using the key store
    return Ok(true);
}
