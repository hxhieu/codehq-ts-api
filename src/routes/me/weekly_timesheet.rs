use actix_web::{get, HttpResponse, Responder};

#[get("/me/timesheet/weekly")]
async fn get() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
