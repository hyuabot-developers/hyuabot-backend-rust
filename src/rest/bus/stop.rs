use crate::error_handler::CustomError;
use crate::model::bus::stop::BusStopItem;
use crate::request::bus::stop::BusStopNameQuery;
use crate::response::bus::stop::{BusStopItemResponse, BusStopListResponse};
use actix_web::get;
use actix_web::web::{Path, Query};
use actix_web::HttpResponse;

#[get("")]
pub async fn get_bus_stop_list(
    stop_query: Query<BusStopNameQuery>,
) -> Result<HttpResponse, CustomError> {
    let stop_list = match stop_query.name {
        Some(ref stop_id) => BusStopItem::find_by_id(stop_id)?,
        None => BusStopItem::find_all()?,
    };
    Ok(HttpResponse::Ok().json(BusStopListResponse::new(stop_list)))
}

#[get("/{stop_id}")]
pub async fn get_bus_stop_by_id(stop_id: Path<i32>) -> Result<HttpResponse, CustomError> {
    let stop_item = BusStopItem::get_one_by_id(&stop_id)?;
    Ok(HttpResponse::Ok().json(BusStopItemResponse::new(stop_item)))
}
