use serde::Serialize;
use crate::model::bus::route::BusRouteItem;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BusRouteListResponse {
    pub route_list: Vec<BusRouteListItem>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BusRouteListItem {
    pub route_id: i32,
    pub route_name: String,
    pub route_type: String
}

impl BusRouteListResponse {
    pub fn new(route_list: Vec<BusRouteItem>) -> Self {
        BusRouteListResponse {
            route_list: route_list.into_iter().map(|route_item| BusRouteListItem::new(route_item)).collect()
        }
    }
}

impl BusRouteListItem {
    pub fn new(route_item: BusRouteItem) -> Self {
        BusRouteListItem {
            route_id: route_item.route_id,
            route_name: route_item.route_name,
            route_type: route_item.route_type_name
        }
    }
}