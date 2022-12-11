use crate::error_handler::CustomError;
use crate::model::shuttle::period::ShuttlePeriodItem;
use crate::model::shuttle::route_stop::ShuttleRouteStopItem;
use crate::model::shuttle::stop::ShuttleStopItem;
use crate::model::shuttle::timetable::{
    ShuttleStopTimeTableItem, ShuttleTimeTableByShuttleStopItem,
};
use crate::request::shuttle::stop::{ShuttleStopItemQuery, ShuttleStopNameQuery};
use crate::response::shuttle::stop::{
    ShuttleRouteStopArrivalResponse, ShuttleRouteStopResponse, ShuttleRouteStopTimetableResponse,
    ShuttleStopItemResponse, ShuttleStopListResponse,
};
use crate::utils::shuttle::get_shuttle_weekday;
use actix_web::web::Query;
use actix_web::HttpResponse;
use actix_web::{get, web};
use std::borrow::Borrow;

#[get("")]
pub async fn get_shuttle_stop(
    stop_query: Query<ShuttleStopNameQuery>,
) -> Result<HttpResponse, CustomError> {
    let stop_list = match stop_query.stop_name {
        Some(ref route_name) => ShuttleStopItem::find_by_name(route_name)?,
        None => ShuttleStopItem::find_all()?,
    };
    Ok(HttpResponse::Ok().json(ShuttleStopListResponse::new(stop_list)))
}

#[get("/{stop_id}")]
pub async fn get_shuttle_stop_by_id(
    stop_id: web::Path<String>,
    stop_item_query: Query<ShuttleStopItemQuery>,
) -> Result<HttpResponse, CustomError> {
    let stop_id = stop_id.into_inner();
    let timetable = ShuttleStopTimeTableItem::get_timetable_by_stop_name(&stop_id)?;
    Ok(HttpResponse::Ok().json(ShuttleStopItemResponse::new(
        stop_id,
        timetable,
        &(get_shuttle_weekday() == "weekdays"),
        &stop_item_query.show_all,
    )))
}

#[get("/{stop_id}/route/{route_id}")]
pub async fn get_shuttle_route_stop_item(
    route_stop_query: web::Path<(String, String)>,
    stop_item_query: Query<ShuttleStopItemQuery>,
) -> Result<HttpResponse, CustomError> {
    let query = route_stop_query.into_inner();
    let timetable = ShuttleStopTimeTableItem::get_timetable_by_stop_name_and_route_name(
        &*query.0.clone(),
        &*query.1.clone(),
    )?;
    Ok(HttpResponse::Ok().json(ShuttleRouteStopResponse::new(
        query.0,
        timetable[0]
            .route_description_korean
            .clone()
            .unwrap_or_default(),
        timetable[1]
            .route_description_english
            .clone()
            .unwrap_or_default(),
        timetable
            .iter()
            .map(|item| {
                (
                    item.weekday,
                    item.departure_time,
                    item.cumulative_time.unwrap_or_default(),
                )
            })
            .collect(),
        &(get_shuttle_weekday() == "weekdays"),
        &stop_item_query.show_all,
    )))
}

#[get("/{stop_id}/route/{route_id}/timetable")]
pub async fn get_shuttle_route_stop_timetable_item(
    route_stop_query: web::Path<(String, String)>,
    stop_item_query: Query<ShuttleStopItemQuery>,
) -> Result<HttpResponse, CustomError> {
    let query = route_stop_query.into_inner();
    let period = ShuttlePeriodItem::get_current_period()?;
    let weekday = get_shuttle_weekday();
    let route_item =
        ShuttleRouteStopItem::get_route_item_by_stop_name(&query.borrow().0, &query.borrow().1)?;
    let timetable = ShuttleTimeTableByShuttleStopItem::get_timetable_by_route_stop_name(
        &period.period_type,
        &(weekday == "weekdays"),
        &route_item,
        &stop_item_query.show_all,
    )
    .unwrap();
    Ok(
        HttpResponse::Ok().json(ShuttleRouteStopTimetableResponse::new(
            &route_item,
            &timetable.iter().collect(),
        )),
    )
}

#[get("/{stop_id}/route/{route_id}/arrival")]
pub async fn get_shuttle_route_stop_arrival_item(
    route_stop_query: web::Path<(String, String)>,
    stop_item_query: Query<ShuttleStopItemQuery>,
) -> Result<HttpResponse, CustomError> {
    let query = route_stop_query.into_inner();
    let period = ShuttlePeriodItem::get_current_period()?;
    let weekday = get_shuttle_weekday();
    let route_item =
        ShuttleRouteStopItem::get_route_item_by_stop_name(&query.borrow().0, &query.borrow().1)?;
    let timetable = ShuttleTimeTableByShuttleStopItem::get_timetable_by_route_stop_name(
        &period.period_type,
        &(weekday == "weekdays"),
        &route_item,
        &stop_item_query.show_all,
    )
    .unwrap();
    Ok(HttpResponse::Ok().json(ShuttleRouteStopArrivalResponse::new(&route_item, timetable)))
}
