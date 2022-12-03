use actix_web::{get, web};
use actix_web::HttpResponse;
use crate::error_handler::CustomError;
use crate::model::shuttle::stop::ShuttleStopItem;
use crate::request::shuttle::stop::ShuttleStopNameQuery;
use crate::response::shuttle::stop::ShuttleStopListResponse;

#[get("/shuttle/stop")]
pub async fn get_shuttle_stop(stop_query: web::Query<ShuttleStopNameQuery>) -> Result<HttpResponse, CustomError> {
    let stop_list = match stop_query.stop_name {
        Some(ref route_name) => ShuttleStopItem::find_by_name(route_name)?,
        None => ShuttleStopItem::find_all()?,
    };
    Ok(HttpResponse::Ok().json(ShuttleStopListResponse::new(stop_list)))
}