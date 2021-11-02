use lazy_static::__Deref;

#[macro_use]
extern crate lazy_static;

mod auth;
mod codehq_ts_cli;
mod config;
mod routes;
mod state;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use crate::auth::middleware::bearer_jwt;
    use actix_cors::Cors;
    use actix_web::{http, middleware::Logger, web, App, HttpServer};
    use actix_web_httpauth::middleware::HttpAuthentication;
    use dotenv::dotenv;

    // Load env vars
    dotenv().ok();

    env_logger::init();

    HttpServer::new(|| {
        // Just assume config can always be loaded otherwise just panic
        let config = config::get().expect("Failed to load the configuration!");
        let allowed_methods = config
            .cors_allowed_methods
            .iter()
            .map(|s| s.deref())
            .collect::<Vec<&str>>();
        let allowed_origins = config.cors_allowed_origins;

        let cors = Cors::default()
            .allowed_origin_fn(move |origin, _| {
                let origin_value = origin.to_str().ok().unwrap().to_string();
                allowed_origins.contains(&origin_value)
            })
            .allowed_methods(allowed_methods)
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(cors)
            // Logger middleware
            .wrap(Logger::default())
            .service(routes::client_config::get_client_config)
            .service(
                // Prefix /api
                web::scope("/api")
                    // Token bearer middleware
                    .wrap(HttpAuthentication::bearer(bearer_jwt))
                    // Routes
                    .service(routes::me::weekly_timesheet::get_now)
                    .service(routes::me::weekly_timesheet::get_date),
            )
    })
    .bind("[::]:8080")?
    .run()
    .await
}
