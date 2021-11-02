use super::response::ApiResponse;
use crate::config::{self, Config};
use actix_web::{get, HttpRequest, HttpResponse, Responder};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct ClientConfiguration {
    auth: Config,
    api_url: Option<String>,
}

#[get("/.well-known/client-configuration")]
async fn get_client_config(req: HttpRequest) -> impl Responder {
    match config::get().ok() {
        Some(cfg) => HttpResponse::Ok().content_type("application/json").body(
            serde_json::to_string(&ApiResponse::<ClientConfiguration> {
                data: Some(ClientConfiguration {
                    auth: cfg,
                    // Get request host
                    api_url: match req.headers().get("host") {
                        Some(host) => host
                            .to_str()
                            .ok()
                            .map(|host_str| format!("{}/api", host_str)),
                        _ => None,
                    },
                }),
                error: None,
            })
            .unwrap(),
        ),
        None => HttpResponse::InternalServerError()
            .content_type("application/json")
            .body(
                serde_json::to_string(&ApiResponse::<String> {
                    data: None,
                    error: Some("Failed to get the configuration".to_string()),
                })
                .unwrap(),
            ),
    }
}
