use std::borrow::Borrow;
use actix_web::get;
use actix_web::web;
use actix_web::HttpResponse;
use crate::error_handler::CustomError;
use crate::model::bus::route::BusRouteItem;
use crate::request::bus::route::BusRouteNameQuery;
use crate::response::bus::route::BusRouteListResponse;

#[get("")]
pub async fn get_bus_route(route_query: web::Query<BusRouteNameQuery>) -> Result<HttpResponse, CustomError> {
    let routes = match route_query.name {
        Some(ref route_name) => BusRouteItem::find_by_name(route_name)?,
        None => BusRouteItem::find_all()?,
    };
    Ok(HttpResponse::Ok().json(BusRouteListResponse::new(routes)))
}