use crate::codehq_ts_cli::get_weekly_timesheet;
use actix_web::{get, web, HttpResponse, Responder};
use chrono::Local;

#[get("/me/timesheet/weekly")]
async fn get_now() -> impl Responder {
    let today = Local::today().format("%d%m%Y").to_string();
    match get_weekly_timesheet("hugh.hoang", &today) {
        Ok(result) => HttpResponse::Ok().body(result),
        Err(err) => HttpResponse::InternalServerError().body(err),
    }
}

#[get("/me/timesheet/weekly/{date}")]
async fn get_date(web::Path(date): web::Path<String>) -> impl Responder {
    match get_weekly_timesheet("hugh.hoang", &date) {
        Ok(result) => HttpResponse::Ok().body(result),
        Err(err) => HttpResponse::InternalServerError().body(err),
    }
}
