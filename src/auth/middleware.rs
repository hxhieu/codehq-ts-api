use actix_web::dev::ServiceRequest;
use actix_web::{error, Error};
use actix_web_httpauth::extractors::bearer::BearerAuth;

use super::jwks;

pub async fn bearer_jwt(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, Error> {
    match jwks::get_jwks().await {
        Ok(key_store) => match jwks::validate_token(credentials.token(), &key_store).await {
            Ok(_) => Ok(req),
            Err(err) => Err(error::ErrorUnauthorized(err)),
        },
        Err(err) => Err(error::ErrorInternalServerError(err)),
    }
}
