use std::borrow::Borrow;
use actix_web::{get, web};
use actix_web::HttpResponse;
use crate::error_handler::CustomError;
use crate::model::shuttle::period::ShuttlePeriodItem;
use crate::model::shuttle::route::ShuttleRouteItem;
use crate::model::shuttle::route_stop::ShuttleRouteStopItem;
use crate::model::shuttle::timetable::ShuttleTimeTableItem;
use crate::request::shuttle::route::ShuttleRouteNameQuery;
use crate::response::shuttle::route::{ShuttleLocationResponse, ShuttleRouteListResponse, ShuttleRouteResponse};
use crate::utils::shuttle::get_shuttle_weekday;

#[get("")]
pub async fn get_shuttle_route(route_query: web::Query<ShuttleRouteNameQuery>) -> Result<HttpResponse, CustomError> {
    // check if route_name is provided in query parameter
    let routes = match route_query.route_name {
        Some(ref route_name) => ShuttleRouteItem::find_by_name(route_name)?,
        None => ShuttleRouteItem::find_all()?,
    };
    Ok(HttpResponse::Ok().json(ShuttleRouteListResponse::new(routes)))
}

#[get("/{route_id}")]
pub async fn get_shuttle_route_by_id(route_id: web::Path<String>) -> Result<HttpResponse, CustomError> {
    let route_id = route_id.into_inner();
    let route = ShuttleRouteItem::get_one_by_name(route_id.borrow())?;
    let period = ShuttlePeriodItem::get_current_period()?;
    let weekday = get_shuttle_weekday();
    let stop_list = ShuttleRouteStopItem::get_stop_list_by_route_name(route_id.borrow())?;
    let timetable = ShuttleTimeTableItem::get_timetable_by_route_name(route_id.borrow(), &period.period_type)?;
    Ok(HttpResponse::Ok().json(ShuttleRouteResponse::new(route, &weekday, &stop_list, &timetable)))
}

#[get("/{route_id}/location")]
pub async fn get_shuttle_location_by_id(route_id: web::Path<String>) -> Result<HttpResponse, CustomError> {
    let route_id = route_id.into_inner();
    let period = ShuttlePeriodItem::get_current_period()?;
    let weekday = get_shuttle_weekday();
    let stop_list = ShuttleRouteStopItem::get_stop_list_by_route_name(route_id.borrow())?;
    let timetable = ShuttleTimeTableItem::get_timetable_by_route_name(route_id.borrow(), &period.period_type)?;
    Ok(HttpResponse::Ok().json(ShuttleLocationResponse::new(&weekday, &stop_list, &timetable)))
}