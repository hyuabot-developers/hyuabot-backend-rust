use actix_web::get;
use actix_web::HttpResponse;
use actix_web::web::{Path, Query};
use chrono::{NaiveDate, Timelike};
use crate::error_handler::CustomError;
use crate::model::cafeteria::menu::RestaurantMenuItem;
use crate::request::cafeteria::menu::MenuQuery;
use crate::response::cafeteria::menu::RestaurantListResponse;

#[get("/campus/{campus_id}/menu")]
pub async fn get_menu_list_by_campus_id(campus_id: Path<i32>, menu_query: Query<MenuQuery>) -> Result<HttpResponse, CustomError> {
    let now = chrono::Local::now().naive_local();
    let feed_date = match &menu_query.date {
        Some(date) => NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap(),
        None => now.date()
    };
    let time_type = match &menu_query.time_type {
        Some(time_type) => time_type,
        None => match now.time().hour() {
            0..=9 => "조식",
            10..=15 => "중식",
            _ => "석식"
        }
    };
    let menu_list = RestaurantMenuItem::find_by_campus_id_and_time(&campus_id, &feed_date, &time_type)?;
    Ok(HttpResponse::Ok().json(RestaurantListResponse::new(menu_list)))
}