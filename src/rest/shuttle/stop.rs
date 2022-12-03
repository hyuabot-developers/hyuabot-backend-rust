use std::borrow::Borrow;
use actix_web::{get, web};
use actix_web::HttpResponse;
use chrono::Datelike;
use crate::error_handler::CustomError;
use crate::model::shuttle::period::{ShuttleHolidayItem, ShuttlePeriodItem};
use crate::model::shuttle::route_stop::ShuttleRouteStopItem;
use crate::model::shuttle::stop::ShuttleStopItem;
use crate::request::shuttle::stop::ShuttleStopNameQuery;
use crate::response::shuttle::stop::{ShuttleStopItemResponse, ShuttleStopListResponse};

#[get("/shuttle/stop")]
pub async fn get_shuttle_stop(stop_query: web::Query<ShuttleStopNameQuery>) -> Result<HttpResponse, CustomError> {
    let stop_list = match stop_query.stop_name {
        Some(ref route_name) => ShuttleStopItem::find_by_name(route_name)?,
        None => ShuttleStopItem::find_all()?,
    };
    Ok(HttpResponse::Ok().json(ShuttleStopListResponse::new(stop_list)))
}

#[get("/shuttle/stop/{stop_id}")]
pub async fn get_shuttle_stop_by_id(stop_id: web::Path<String>) -> Result<HttpResponse, CustomError> {
    let stop_id = stop_id.into_inner();
    let stop = ShuttleStopItem::get_one_by_name(stop_id.borrow())?;
    let period = ShuttlePeriodItem::get_current_period()?;
    let weekday = match ShuttleHolidayItem::get_holiday_by_date(chrono::Local::now().naive_local()) {
        Ok(holiday_item) => holiday_item.holiday_type,
        Err(_) => {
            if chrono::Local::now().weekday() == chrono::Weekday::Sat || chrono::Local::now().weekday() == chrono::Weekday::Sun {
                "weekends".to_string()
            } else {
                "weekdays".to_string()
            }
        }
    };
    let route_list = ShuttleRouteStopItem::get_route_list_by_stop_name(stop_id.borrow())?;
    // let timetable = ShuttleTimeTableItem::get_timetable_by_route_name(route_id.borrow(), &period.period_type)?;
    Ok(HttpResponse::Ok().json(ShuttleStopItemResponse::new(
        stop, &route_list
    )))
}