use actix_web::get;
use actix_web::HttpResponse;
use actix_web::web::Path;
use crate::error_handler::CustomError;
use crate::model::cafeteria::restaurant::RestaurantItem;
use crate::response::cafeteria::restaurant::RestaurantListResponse;

#[get("/campus/{campus_id}")]
pub async fn get_restaurant_list_by_campus_id(campus_id: Path<i32>) -> Result<HttpResponse, CustomError> {
    let restaurant_list = RestaurantItem::find_by_campus_id(&campus_id)?;
    Ok(HttpResponse::Ok().json(RestaurantListResponse::new(restaurant_list)))
}