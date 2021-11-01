use crate::{codehq_ts_cli::get_weekly_timesheet, state::RequestContext};
use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use chrono::Local;

#[get("/me/timesheet/weekly")]
async fn get_now(req: HttpRequest) -> impl Responder {
    let today = Local::today().format("%d%m%Y").to_string();
    let user = RequestContext::new(&req).user;
    match get_weekly_timesheet(&user, &today) {
        Ok(result) => HttpResponse::Ok()
            .content_type("application/json")
            .body(result),
        Err(err) => HttpResponse::InternalServerError().body(err),
    }
}

#[get("/me/timesheet/weekly/{date}")]
async fn get_date(web::Path(date): web::Path<String>, req: HttpRequest) -> impl Responder {
    let user = RequestContext::new(&req).user;
    match get_weekly_timesheet(&user, &date) {
        Ok(result) => HttpResponse::Ok()
            .content_type("application/json")
            .body(result),
        Err(err) => HttpResponse::InternalServerError().body(err),
    }
}
