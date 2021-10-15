mod routes;

use actix_web::{middleware::Logger, web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    HttpServer::new(|| {
        App::new()
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
