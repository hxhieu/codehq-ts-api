extern crate alcoholic_jwt;

use alcoholic_jwt::JWKS;
use dotenv::dotenv;
use log::info;
use openidconnect::{
    core::CoreProviderMetadata,
    http::{HeaderMap, Method},
    reqwest::http_client,
    HttpRequest, IssuerUrl,
};
use std::{env, str, sync::RwLock};

// https://stackoverflow.com/questions/61159698/update-re-initialize-a-var-defined-in-lazy-static
lazy_static! {
    static ref KEY_STORE: RwLock<Option<JWKS>> = RwLock::new(None);
}

pub async fn get_jwks() -> Result<JWKS, String> {
    // Get from the cache first
    if let Some(key_store) = KEY_STORE.read().unwrap().to_owned() {
        return Ok(key_store);
    }

    // Otherwise fetch the JWKS for the 1st time

    info!("No JWKS in cache, fetch it now");

    dotenv().ok(); // Read from .env if there is one

    let authority_url =
        IssuerUrl::new(env::var("CODEHQ_TS_API_AUTH_AUTHORITY").unwrap_or("".to_string()))
            .map_err(|err| format!("Invalid authority URL. {}", err))?;

    let metadata = CoreProviderMetadata::discover(&authority_url, http_client)
        .map_err(|err| format!("Failed to fetch authority metadata. {}", err))?;

    // HACK: Can't the reqwest to work by itself, so
    // using openidconnect http_request instead...
    let request = HttpRequest {
        url: metadata.jwks_uri().url().to_owned(),
        method: Method::GET,
        headers: HeaderMap::new(),
        body: Vec::new(),
    };

    let response = http_client(request)
        .map_err(|err| format!("Failed to fetch JWKS from the authority. {}", err))?;
    let key_store = serde_json::from_slice::<JWKS>(&response.body)
        .map_err(|err| format!("Failed to deserialise the JWKS. {}", err))?;

    // Update the cache
    let mut new_key_store = KEY_STORE.write().unwrap();
    *new_key_store = Some(key_store.clone());
    // Return cache value
    return Ok(key_store);
}

pub async fn validate_token(token: &str, key_store: &JWKS) -> Result<bool, String> {
    if token.len() == 0 {
        return Err("Bearer token is required".to_string());
    }
    print!("{:?}", key_store);
    // TODO: Actually using the key store
    Ok(true)
}
