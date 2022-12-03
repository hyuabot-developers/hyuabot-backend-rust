use std::borrow::Borrow;
use actix_web::{get, web};
use actix_web::HttpResponse;
use crate::error_handler::CustomError;
use crate::model::shuttle::route::ShuttleRouteItem;
use crate::model::shuttle::route_stop::ShuttleRouteStopItem;
use crate::model::shuttle::timetable::ShuttleTimeTableItem;
use crate::request::shuttle::route::ShuttleRouteNameQuery;
use crate::response::shuttle::route::{ShuttleRouteListResponse, ShuttleRouteResponse};

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
    let stop_list = ShuttleRouteStopItem::get_stop_list_by_route_name(route_id.borrow())?;
    let timetable = ShuttleTimeTableItem::get_timetable_by_route_name(route_id.borrow())?;
    Ok(HttpResponse::Ok().json(ShuttleRouteResponse::new(route, &stop_list, &timetable)))
}