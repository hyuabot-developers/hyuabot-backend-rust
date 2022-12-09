use serde::Serialize;
use crate::model::library::room::ReadingRoomItem;


#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReadingRoomListResponse {
    pub room_list: Vec<ReadingRoomItemResponse>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReadingRoomItemResponse {
    pub name: String,
    pub id: i32,
    pub is_active: bool,
    pub active_total: i32,
    pub occupied: i32,
    pub available: i32,
}

impl ReadingRoomListResponse {
    pub fn new(rooms: Vec<ReadingRoomItem>) -> Self {
        let room_list = rooms
            .into_iter()
            .map(|room| ReadingRoomItemResponse::new(room))
            .collect();
        Self { room_list }
    }
}

impl ReadingRoomItemResponse {
    pub fn new(room: ReadingRoomItem) -> Self {
        Self {
            name: room.room_name,
            id: room.room_id,
            is_active: room.is_active,
            active_total: room.active_total,
            occupied: room.occupied,
            available: room.available,
        }
    }
}