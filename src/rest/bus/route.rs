use std::borrow::Borrow;
use actix_web::get;
use actix_web::web;
use actix_web::HttpResponse;
use crate::error_handler::CustomError;
use crate::model::bus::route::BusRouteItem;
use crate::request::bus::route::BusRouteNameQuery;
use crate::response::bus::route::{BusRouteItemResponse, BusRouteListResponse};

#[get("")]
pub async fn get_bus_route(route_query: web::Query<BusRouteNameQuery>) -> Result<HttpResponse, CustomError> {
    let routes = match route_query.name {
        Some(ref route_name) => BusRouteItem::find_by_name(route_name)?,
        None => BusRouteItem::find_all()?,
    };
    Ok(HttpResponse::Ok().json(BusRouteListResponse::new(routes)))
}

#[get("/{route_id}")]
pub async fn get_bus_route_by_id(route_id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let route_id = route_id.into_inner();
    let route = BusRouteItem::get_by_id(route_id.borrow())?;
    Ok(HttpResponse::Ok().json(BusRouteItemResponse::new(route)))
}