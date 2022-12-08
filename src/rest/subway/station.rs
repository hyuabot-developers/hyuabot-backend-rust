use actix_web::get;
use actix_web::HttpResponse;
use actix_web::web::Path;
use actix_web::web::Query;
use crate::error_handler::CustomError;
use crate::model::subway::station::SubwayStationItem;
use crate::request::subway::station::SubwayStationQuery;
use crate::response::subway::station::SubwayStationListResponse;

#[get("")]
pub async fn get_subway_station_list(station_query: Query<SubwayStationQuery>) -> Result<HttpResponse, CustomError> {
    let station_list = match station_query.name {
        Some(ref station_name) => SubwayStationItem::find_by_name(station_name)?,
        None => SubwayStationItem::find_all()?,
    };
    Ok(HttpResponse::Ok().json(SubwayStationListResponse::new(station_list)))
}
