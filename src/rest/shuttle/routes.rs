use actix_web::{get, web};
use actix_web::HttpResponse;
use crate::error_handler::CustomError;
use crate::model::shuttle::route::ShuttleRouteItem;
use crate::request::shuttle::route::ShuttleRouteNameQuery;
use crate::response::shuttle::route::ShuttleRouteList;

#[get("/shuttle/route")]
pub async fn get_shuttle_route(route_query: web::Query<ShuttleRouteNameQuery>) -> Result<HttpResponse, CustomError> {
    // check if route_name is provided in query parameter
    let routes = match route_query.route_name {
        Some(ref route_name) => ShuttleRouteItem::find_by_name(route_name)?,
        None => ShuttleRouteItem::find_all()?,
    };
    Ok(HttpResponse::Ok().json(ShuttleRouteList::new(routes)))
}