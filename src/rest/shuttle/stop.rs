use std::borrow::Borrow;
use actix_web::{get, web};
use actix_web::HttpResponse;
use actix_web::web::Query;
use crate::error_handler::CustomError;
use crate::model::shuttle::period::ShuttlePeriodItem;
use crate::model::shuttle::route_stop::ShuttleRouteStopItem;
use crate::model::shuttle::stop::ShuttleStopItem;
use crate::model::shuttle::timetable::ShuttleTimeTableByShuttleStopItem;
use crate::request::shuttle::stop::{ShuttleStopItemQuery, ShuttleStopNameQuery};
use crate::response::shuttle::stop::{ShuttleRouteStopArrivalResponse, ShuttleRouteStopResponse, ShuttleRouteStopTimetableResponse, ShuttleStopItemResponse, ShuttleStopListResponse};
use crate::utils::shuttle::get_shuttle_weekday;

#[get("")]
pub async fn get_shuttle_stop(stop_query: Query<ShuttleStopNameQuery>) -> Result<HttpResponse, CustomError> {
    let stop_list = match stop_query.stop_name {
        Some(ref route_name) => ShuttleStopItem::find_by_name(route_name)?,
        None => ShuttleStopItem::find_all()?,
    };
    Ok(HttpResponse::Ok().json(ShuttleStopListResponse::new(stop_list)))
}

#[get("/{stop_id}")]
pub async fn get_shuttle_stop_by_id(stop_id: web::Path<String>, stop_item_query: Query<ShuttleStopItemQuery>) -> Result<HttpResponse, CustomError> {
    let stop_id = stop_id.into_inner();
    let stop = ShuttleStopItem::get_one_by_name(stop_id.borrow())?;
    let period = ShuttlePeriodItem::get_current_period()?;
    let weekday = get_shuttle_weekday();
    let route_list = ShuttleRouteStopItem::get_route_list_by_stop_name(stop_id.borrow())?;
    let limit = stop_item_query.limit.unwrap_or_else(|| 999);
    Ok(HttpResponse::Ok().json(ShuttleStopItemResponse::new(
        stop, &route_list, &period, &(weekday == "weekdays"), &limit, &stop_item_query.show_all,
    )))
}

#[get("/{stop_id}/route/{route_id}")]
pub async fn get_shuttle_route_stop_item(route_stop_query: web::Path<(String, String)>, stop_item_query: Query<ShuttleStopItemQuery>) -> Result<HttpResponse, CustomError> {
    let query = route_stop_query.into_inner();
    let period = ShuttlePeriodItem::get_current_period()?;
    let weekday = get_shuttle_weekday();
    let route_item = ShuttleRouteStopItem::get_route_item_by_stop_name(&query.borrow().0, &query.borrow().1)?;
    let limit = stop_item_query.limit.unwrap_or_else(|| 999);
    let timetable = ShuttleTimeTableByShuttleStopItem::get_timetable_by_route_stop_name(
        &period.period_type, &(weekday == "weekdays"), &route_item, &limit, &stop_item_query.show_all,
    ).unwrap();
    Ok(HttpResponse::Ok().json(ShuttleRouteStopResponse::new(
        &route_item, &timetable.iter().collect()
    )))
}

#[get("/{stop_id}/route/{route_id}/timetable")]
pub async fn get_shuttle_route_stop_timetable_item(route_stop_query: web::Path<(String, String)>, stop_item_query: Query<ShuttleStopItemQuery>) -> Result<HttpResponse, CustomError> {
    let query = route_stop_query.into_inner();
    let period = ShuttlePeriodItem::get_current_period()?;
    let weekday = get_shuttle_weekday();
    let route_item = ShuttleRouteStopItem::get_route_item_by_stop_name(&query.borrow().0, &query.borrow().1)?;
    let limit = stop_item_query.limit.unwrap_or_else(|| 999);
    let timetable = ShuttleTimeTableByShuttleStopItem::get_timetable_by_route_stop_name(
        &period.period_type, &(weekday == "weekdays"), &route_item, &limit, &stop_item_query.show_all,
    ).unwrap();
    Ok(HttpResponse::Ok().json(ShuttleRouteStopTimetableResponse::new(
        &route_item, &timetable.iter().collect()
    )))
}

#[get("/{stop_id}/route/{route_id}/arrival")]
pub async fn get_shuttle_route_stop_arrival_item(route_stop_query: web::Path<(String, String)>, stop_item_query: Query<ShuttleStopItemQuery>) -> Result<HttpResponse, CustomError> {
    let query = route_stop_query.into_inner();
    let period = ShuttlePeriodItem::get_current_period()?;
    let weekday = get_shuttle_weekday();
    let route_item = ShuttleRouteStopItem::get_route_item_by_stop_name(&query.borrow().0, &query.borrow().1)?;
    let limit = stop_item_query.limit.unwrap_or_else(|| 999);
    let timetable = ShuttleTimeTableByShuttleStopItem::get_timetable_by_route_stop_name(
        &period.period_type, &(weekday == "weekdays"), &route_item, &limit, &stop_item_query.show_all,
    ).unwrap();
    Ok(HttpResponse::Ok().json(ShuttleRouteStopArrivalResponse::new(
        &route_item, &timetable.iter().collect()
    )))
}