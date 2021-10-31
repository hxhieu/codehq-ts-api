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
    use actix_web::{middleware::Logger, web, App, HttpServer};
    use actix_web_httpauth::middleware::HttpAuthentication;
    use dotenv::dotenv;

    // Load env vars
    dotenv().ok();

    env_logger::init();

    HttpServer::new(|| {
        App::new()
            // Logger middleware
            .wrap(Logger::default())
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
