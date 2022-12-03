use serde::Serialize;
use crate::model::shuttle::stop::ShuttleStopItem;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ShuttleStopListResponse {
    pub stop_list: Vec<ShuttleStopItem>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ShuttleStopItemResponse {
    pub stop_name: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

impl ShuttleStopListResponse {
    pub fn new(stop_list: Vec<ShuttleStopItem>) -> Self {
        ShuttleStopListResponse { stop_list }
    }
}

impl ShuttleStopItemResponse {
    pub fn new(stop_name: String, latitude: Option<f64>, longitude: Option<f64>) -> Self {
        ShuttleStopItemResponse { stop_name, latitude, longitude }
    }
}

