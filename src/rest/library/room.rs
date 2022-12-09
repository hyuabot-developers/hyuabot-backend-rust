use actix_web::get;
use actix_web::HttpResponse;
use actix_web::web::Path;
use crate::error_handler::CustomError;
use crate::model::library::room::ReadingRoomItem;
use crate::response::library::room::{ReadingRoomItemResponse, ReadingRoomListResponse};

#[get("/campus/{campus_id}")]
pub async fn get_room_list_by_campus_id(campus_id: Path<i32>) -> Result<HttpResponse, CustomError> {
    let room_list = ReadingRoomItem::find_by_campus_id(&campus_id)?;
    Ok(HttpResponse::Ok().json(ReadingRoomListResponse::new(room_list)))
}

#[get("/campus/{campus_id}/room/{room_id}")]
pub async fn get_room_by_room_id(room_query: Path<(i32, i32)>) -> Result<HttpResponse, CustomError> {
    let query = room_query.into_inner();
    let room = ReadingRoomItem::get_by_id(&query.0, &query.1)?;
    Ok(HttpResponse::Ok().json(ReadingRoomItemResponse::new(room)))
}
