use super::jwks;
use crate::{config, state::RequestContext};
use actix_web::{dev::ServiceRequest, error, Error};
use actix_web_httpauth::extractors::bearer::BearerAuth;

/// actix_web_httpauth specific bearer middleware
pub async fn bearer_jwt(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, Error> {
    // Load the auth configuration to be passed around
    let config = config::get().map_err(|err| {
        error::ErrorInternalServerError(format!("Failed to load configuration. {}", err,))
    })?;

    let key_store = jwks::get_jwks(Some(&config))
        .await
        .map_err(error::ErrorInternalServerError)?;

    let claims = jwks::validate_token(credentials.token(), &key_store, Some(&config))
        .await
        .map_err(error::ErrorUnauthorized)?;

    // UPN checking, make sure the users from the correct organisation

    let upn = claims.get("upn");
    let upn_segs = upn.split('@').collect::<Vec<&str>>();

    if upn.is_empty() || upn_segs.len() != 2 {
        return Err(error::ErrorUnauthorized(
            "Token UPN is not valid for this request.",
        ));
    }

    let mut valid = false;
    for allowed_domain in config.auth_allowed_domains {
        if allowed_domain == upn_segs[1] {
            valid = true;
            break;
        }
    }

    if !valid {
        return Err(error::ErrorUnauthorized(format!(
            "Token domain '{}' is not valid for this request.",
            upn_segs[1]
        )));
    }

    // Update the request context
    RequestContext::set_user(&req, upn_segs[0]);

    Ok(req)
}
