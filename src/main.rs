#[macro_use]
extern crate lazy_static;

mod auth;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use crate::auth::middleware::bearer_jwt;
    use actix_web::{middleware::Logger, web, App, HttpServer};
    use actix_web_httpauth::middleware::HttpAuthentication;

    env_logger::init();

    HttpServer::new(|| {
        App::new()
            // Token bearer middleware
            .wrap(HttpAuthentication::bearer(bearer_jwt))
            // Logger middleware
            .wrap(Logger::default())
            .service(
                // Prefix /api
                web::scope("/api")
                    // Routes
                    .service(routes::me::weekly_timesheet::get),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
