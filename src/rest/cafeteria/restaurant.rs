use crate::error_handler::CustomError;
use crate::model::cafeteria::menu::MenuItem;
use crate::model::cafeteria::restaurant::RestaurantItem;
use crate::request::cafeteria::restaurant::RestaurantMenuQuery;
use crate::response::cafeteria::restaurant::{RestaurantItemResponse, RestaurantListResponse};
use actix_web::get;
use actix_web::web::{Path, Query};
use actix_web::HttpResponse;
use chrono::NaiveDate;

#[get("/campus/{campus_id}/restaurant")]
pub async fn get_restaurant_list_by_campus_id(
    campus_id: Path<i32>,
) -> Result<HttpResponse, CustomError> {
    let restaurant_list = RestaurantItem::find_by_campus_id(&campus_id)?;
    Ok(HttpResponse::Ok().json(RestaurantListResponse::new(restaurant_list)))
}

#[get("/campus/{campus_id}/restaurant/{restaurant_id}")]
pub async fn get_restaurant_item_by_id(
    restaurant_query: Path<(i32, i32)>,
    date_query: Query<RestaurantMenuQuery>,
) -> Result<HttpResponse, CustomError> {
    let query = restaurant_query.into_inner();
    let restaurant_item = RestaurantItem::get_by_id(&query.0, &query.1)?;
    let feed_date = match &date_query.date {
        Some(date) => NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap(),
        None => chrono::Local::now().naive_local().date(),
    };
    let menu_list = MenuItem::find_by_restaurant_id(&query.1, &feed_date)?;

    Ok(HttpResponse::Ok().json(RestaurantItemResponse::new(restaurant_item, menu_list)))
}
