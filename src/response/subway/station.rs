use chrono::NaiveDateTime;
use serde::Serialize;
use crate::model::subway::station::SubwayStationItem;


#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubwayStationListResponse {
    pub station_list: Vec<SubwayStationListItem>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubwayStationListItem {
    pub station_id: String,
    pub route_id: i32,
    pub station_name: String,
}

impl SubwayStationListResponse {
    pub fn new(station_list: Vec<SubwayStationItem>) -> Self {
        Self {
            station_list: station_list.into_iter().map(|stop_item| SubwayStationListItem::new(stop_item)).collect()
        }
    }
}

impl SubwayStationListItem {
    pub fn new(station_item: SubwayStationItem) -> Self {
        Self {
            station_id: station_item.station_id,
            route_id: station_item.route_id,
            station_name: station_item.station_name,
        }
    }
}
