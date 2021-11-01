extern crate alcoholic_jwt;

use crate::config::{self, Config};
use alcoholic_jwt::{token_kid, validate, Validation, ValidationError, JWKS};
use log::{error, info};
use openidconnect::{
    core::CoreProviderMetadata,
    http::{HeaderMap, Method},
    reqwest::http_client,
    HttpRequest, IssuerUrl,
};
use std::{collections::HashMap, str, sync::RwLock};

use super::token_claims::TokenClaims;

// https://stackoverflow.com/questions/61159698/update-re-initialize-a-var-defined-in-lazy-static
lazy_static! {
    static ref KEY_STORE: RwLock<Option<JWKS>> = RwLock::new(None);
}

/// Try to load the config from the passed in configuration, if None the load it from the cache
fn load_config(auth_config: Option<&Config>) -> Result<Config, String> {
    match auth_config {
        Some(cfg) => Ok(cfg.clone()),
        None => Ok(config::get().map_err(|err| format!("Failed to load configuration. {}", err))?),
    }
}

pub async fn get_jwks(auth_config: Option<&Config>) -> Result<JWKS, String> {
    // Get from the cache first
    if let Some(key_store) = KEY_STORE.read().unwrap().to_owned() {
        return Ok(key_store);
    }

    // Otherwise fetch the JWKS for the 1st time

    info!("No JWKS in cache, fetch it now");

    let config = load_config(auth_config).map_err(|e| e)?;

    let authority_url = IssuerUrl::new(config.auth_issuer)
        .map_err(|err| format!("Invalid authority URL. {}", err))?;

    let metadata = CoreProviderMetadata::discover(&authority_url, http_client).map_err(|err| {
        format!(
            "Failed to fetch authority metadata, URL: {:?}. {}",
            authority_url, err,
        )
    })?;

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
    Ok(key_store)
}

pub async fn validate_token(
    token: &str,
    key_store: &JWKS,
    auth_config: Option<&Config>,
) -> Result<TokenClaims, String> {
    if token.is_empty() {
        return Err("Missing bearer token.".to_string());
    }

    let config = load_config(auth_config).map_err(|e| e)?;

    let validations = vec![
        Validation::SubjectPresent,
        Validation::Issuer(config.auth_issuer),
        Validation::Audience(config.auth_client_id),
        Validation::NotExpired,
    ];

    let error_message = "Token validation failed.";

    let kid = token_kid(token).map_err(|_| {
        error!(
            "{} {}",
            error_message, "Invalid JWT or no 'kid' claim present in token."
        );
        error_message
    })?;

    if let Some(jwk) = key_store.find(&kid.unwrap()) {
        let result = validate(token, jwk, validations).map_err(|err| match err {
            ValidationError::InvalidBase64(derr) => {
                error!("{} {}", error_message, derr);
                error_message
            }
            ValidationError::InvalidClaims(cerr) => {
                error!("{} {:?}", error_message, cerr);
                error_message
            }
            ValidationError::InvalidComponents => {
                error!("{} {}", error_message, "Invalid JWT.");
                error_message
            }
            ValidationError::InvalidJWK => {
                error!("{} {}", error_message, "Invalid JWK.");
                error_message
            }
            ValidationError::InvalidSignature => {
                error!("{} {}", error_message, "Invalid signature.");
                error_message
            }
            ValidationError::JSON(jerr) => {
                error!("{} {}", error_message, jerr);
                error_message
            }
            _ => error_message,
        })?;
        let mut claims = HashMap::new();
        for (key, value) in result.claims.as_object().unwrap() {
            claims.insert(
                key.to_string(),
                match value.as_str() {
                    Some(v) => v.to_string(),
                    None => "".to_string(),
                },
            );
        }
        Ok(TokenClaims::new(result))
    } else {
        error!("{} {}", error_message, "Specified key not found in set.");
        Err(error_message.to_string())
    }
}
