extern crate alcoholic_jwt;

use alcoholic_jwt::JWKS;
use dotenv::dotenv;
use openidconnect::core::CoreProviderMetadata;
use openidconnect::reqwest::http_client;
use openidconnect::IssuerUrl;
use std::env;
use std::sync::RwLock;

// https://stackoverflow.com/questions/61159698/update-re-initialize-a-var-defined-in-lazy-static
lazy_static! {
    static ref KEY_STORE: RwLock<Option<JWKS>> = RwLock::new(None);
}

pub async fn get_jwks() -> Result<JWKS, String> {
    if let Some(key_store) = KEY_STORE.read().unwrap().to_owned() {
        return Ok(key_store);
    }

    dotenv().ok();
    let authority_url = env::var("CODEHQ_TS_API_AUTH_AUTHORITY").unwrap_or("".to_string());
    let client_id = env::var("CODEHQ_TS_API_AUTH_CLIENT_ID").unwrap_or("".to_string());
    if authority_url.len() == 0 || client_id.len() == 0 {
        return Err("Auth authority is not configured properly".to_string());
    }

    let issuer_url = IssuerUrl::new(authority_url).expect("Invalid issuer URL");
    let provider_metadata = CoreProviderMetadata::discover(&issuer_url, http_client);
    match provider_metadata {
        Ok(metadata) => {
            let jwks_url = metadata.jwks_uri().to_string();
            let client = reqwest::Client::new();
            match client.get(jwks_url).send().await {
                Ok(res) => match res.json::<JWKS>().await {
                    Ok(key_store) => {
                        // let mut new_key_store = KEY_STORE.write().unwrap();
                        // *new_key_store = Some(key_store);
                        Ok(key_store)
                    }
                    Err(_) => Err("Failed to deserialise JWKS".to_string()),
                },
                Err(_) => Err("Failed to fetch JWKS".to_string()),
            }
        }
        Err(_) => return Err("Failed to fetch authority metadata".to_string()),
    }
}

pub async fn validate_token(token: &str, key_store: &JWKS) -> Result<bool, String> {
    if token.len() == 0 {
        return Err("Bearer token is required".to_string());
    }
    // TODO: Actually using the key store
    Ok(true)
}
