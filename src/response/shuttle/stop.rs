use serde::Serialize;
use crate::model::shuttle::stop::ShuttleStopItem;

#[derive(Serialize)]
pub struct ShuttleStopListResponse {
    pub stop_list: Vec<ShuttleStopItem>,
}

impl ShuttleStopListResponse {
    pub fn new(stop_list: Vec<ShuttleStopItem>) -> Self {
        ShuttleStopListResponse { stop_list }
    }
}
