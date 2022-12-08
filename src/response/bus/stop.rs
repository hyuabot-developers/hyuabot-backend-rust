use serde::Serialize;
use crate::model::bus::stop::BusStopItem;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BusStopListResponse {
    pub stop_list: Vec<BusStopListItem>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BusStopListItem {
    pub stop_id: i32,
    pub stop_name: String,
    pub location: BusStopLocation,
}

#[derive(Serialize)]
pub struct BusStopLocation {
    pub latitude: f64,
    pub longitude: f64
}

impl BusStopListResponse {
    pub fn new(stop_list: Vec<BusStopItem>) -> Self {
        Self {
            stop_list: stop_list.into_iter().map(|stop_item| BusStopListItem::new(stop_item)).collect()
        }
    }
}

impl BusStopListItem {
    pub fn new(stop_item: BusStopItem) -> Self {
        Self {
            stop_id: stop_item.stop_id,
            stop_name: stop_item.stop_name.unwrap(),
            location: BusStopLocation::new(stop_item.latitude, stop_item.longitude),
        }
    }
}

impl BusStopLocation {
    pub fn new(latitude: f64, longitude: f64) -> Self {
        Self {
            latitude,
            longitude,
        }
    }
}