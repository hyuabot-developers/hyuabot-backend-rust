use std::borrow::Borrow;
use actix_web::{get, web};
use actix_web::HttpResponse;
use chrono::Datelike;
use crate::error_handler::CustomError;
use crate::model::shuttle::period::{ShuttleHolidayItem, ShuttlePeriodItem};
use crate::model::shuttle::route::ShuttleRouteItem;
use crate::model::shuttle::route_stop::ShuttleRouteStopItem;
use crate::model::shuttle::timetable::ShuttleTimeTableItem;
use crate::request::shuttle::route::ShuttleRouteNameQuery;
use crate::response::shuttle::route::{ShuttleLocationResponse, ShuttleRouteListResponse, ShuttleRouteResponse};

#[get("/shuttle/route")]
pub async fn get_shuttle_route(route_query: web::Query<ShuttleRouteNameQuery>) -> Result<HttpResponse, CustomError> {
    // check if route_name is provided in query parameter
    let routes = match route_query.route_name {
        Some(ref route_name) => ShuttleRouteItem::find_by_name(route_name)?,
        None => ShuttleRouteItem::find_all()?,
    };
    Ok(HttpResponse::Ok().json(ShuttleRouteListResponse::new(routes)))
}

#[get("/shuttle/route/{route_id}")]
pub async fn get_shuttle_route_by_id(route_id: web::Path<String>) -> Result<HttpResponse, CustomError> {
    let route_id = route_id.into_inner();
    let route = ShuttleRouteItem::get_one_by_name(route_id.borrow())?;
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
    let stop_list = ShuttleRouteStopItem::get_stop_list_by_route_name(route_id.borrow())?;
    let timetable = ShuttleTimeTableItem::get_timetable_by_route_name(route_id.borrow(), &period.period_type)?;
    Ok(HttpResponse::Ok().json(ShuttleRouteResponse::new(route, &weekday, &stop_list, &timetable)))
}

#[get("/shuttle/route/{route_id}/location")]
pub async fn get_shuttle_location_by_id(route_id: web::Path<String>) -> Result<HttpResponse, CustomError> {
    let route_id = route_id.into_inner();
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
    let stop_list = ShuttleRouteStopItem::get_stop_list_by_route_name(route_id.borrow())?;
    let timetable = ShuttleTimeTableItem::get_timetable_by_route_name(route_id.borrow(), &period.period_type)?;
    Ok(HttpResponse::Ok().json(ShuttleLocationResponse::new(&weekday, &stop_list, &timetable)))
}