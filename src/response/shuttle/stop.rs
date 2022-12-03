use serde::Serialize;
use crate::model::shuttle::stop::ShuttleStopItem;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ShuttleStopListResponse {
    pub stop_list: Vec<ShuttleStopItemResponse>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ShuttleStopItemResponse {
    pub stop_name: String,
    pub location: Option<ShuttleStopLocationResponse>,
}

#[derive(Serialize)]
pub struct ShuttleStopLocationResponse {
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

impl ShuttleStopListResponse {
    pub fn new(stop_list: Vec<ShuttleStopItem>) -> Self {
        ShuttleStopListResponse {
            stop_list: stop_list.into_iter().map(|stop| ShuttleStopItemResponse::new(stop)).collect()
        }
    }
}

impl ShuttleStopItemResponse {
    pub fn new(stop_item: ShuttleStopItem) -> Self {
        ShuttleStopItemResponse {
            stop_name: stop_item.stop_name,
            location: ShuttleStopLocationResponse::new(stop_item.latitude, stop_item.longitude),
        }
    }
}

impl ShuttleStopLocationResponse {
    pub fn new(latitude: Option<f64>, longitude: Option<f64>) -> Option<Self> {
        if latitude.is_some() && longitude.is_some() {
            Some(ShuttleStopLocationResponse {
                latitude,
                longitude,
            })
        } else {
            None
        }
    }
}