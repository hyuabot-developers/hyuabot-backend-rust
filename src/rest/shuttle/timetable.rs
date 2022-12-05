use actix_web::{get, HttpResponse};
use crate::error_handler::CustomError;
use crate::model::shuttle::stop::ShuttleStopItem;
use crate::response::shuttle::timetable::{ShuttleArrivalListResponse, ShuttleTimetableListResponse};

#[get("/shuttle/timetable")]
pub async fn get_shuttle_timetable() -> Result<HttpResponse, CustomError> {
    let stop_list = ShuttleStopItem::find_all()?;
    Ok(HttpResponse::Ok().json(ShuttleTimetableListResponse::new(&stop_list)))
}

#[get("/shuttle/arrival")]
pub async fn get_shuttle_arrival() -> Result<HttpResponse, CustomError> {
    let stop_list = ShuttleStopItem::find_all()?;
    Ok(HttpResponse::Ok().json(ShuttleArrivalListResponse::new(&stop_list)))
}