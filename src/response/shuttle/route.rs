use std::ops::Add;
use chrono::Duration;
use serde::Serialize;
use crate::model::shuttle::route::ShuttleRouteItem;
use crate::model::shuttle::route_stop::ShuttleRouteStopItem;
use crate::model::shuttle::timetable::ShuttleTimeTableItem;

#[derive(Serialize)]
pub struct ShuttleRouteListResponse {
    pub routes: Vec<ShuttleRouteItem>,
}

impl ShuttleRouteListResponse {
    pub fn new(routes: Vec<ShuttleRouteItem>) -> Self {
        ShuttleRouteListResponse { routes }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ShuttleRouteResponse {
    pub route_name: String,
    pub description: ShuttleDescriptionItem,
    pub stop_list: Vec<ShuttleRouteStopItemResponse>,
}

#[derive(Serialize)]
pub struct ShuttleDescriptionItem {
    pub korean: String,
    pub english: String,
}

#[derive(Serialize)]
pub struct ShuttleRouteStopItemResponse {
    pub stop_name: String,
    pub stop_order: i32,
    pub weekdays: ShuttleFirstLastTimeItem,
    pub weekends: ShuttleFirstLastTimeItem,
}

#[derive(Serialize)]
pub struct ShuttleFirstLastTimeItem {
    pub first: String,
    pub last: String,
}

impl ShuttleRouteResponse {
    pub fn new(route: ShuttleRouteItem, stop_items: &Vec<ShuttleRouteStopItem>, timetable: &Vec<ShuttleTimeTableItem>) -> Self {
        let weekdays_shuttle = timetable.iter().filter(|item| item.weekday).collect::<Vec<&ShuttleTimeTableItem>>();
        let weekends_shuttle = timetable.iter().filter(|item| !(item.weekday)).collect::<Vec<&ShuttleTimeTableItem>>();
        let mut stop_list = Vec::new();
        let _ = stop_items.iter().map(
            |stop_item| {
                stop_list.push(ShuttleRouteStopItemResponse {
                    stop_name: stop_item.stop_name.clone(),
                    stop_order: stop_item.stop_order.unwrap(),
                    weekdays: ShuttleFirstLastTimeItem {
                        first: match weekdays_shuttle.first() {
                            Some(item) => item.departure_time.add(Duration::minutes(stop_item.cumulative_time.unwrap() as i64)).to_string(),
                            None => "00:00:00".to_string(),
                        },
                        last: match weekdays_shuttle.last() {
                            Some(item) => item.departure_time.add(Duration::minutes(stop_item.cumulative_time.unwrap() as i64)).to_string(),
                            None => "00:00:00".to_string(),
                        },
                    },
                    weekends: ShuttleFirstLastTimeItem {
                        first: match weekends_shuttle.first() {
                            Some(item) => item.departure_time.add(Duration::minutes(stop_item.cumulative_time.unwrap() as i64)).to_string(),
                            None => "00:00:00".to_string(),
                        },
                        last: match weekends_shuttle.last() {
                            Some(item) => item.departure_time.add(Duration::minutes(stop_item.cumulative_time.unwrap() as i64)).to_string(),
                            None => "00:00:00".to_string(),
                        },
                    },
                });
            }
        ).collect::<Vec<()>>();
        ShuttleRouteResponse {
            route_name: route.route_name,
            description: ShuttleDescriptionItem {
                korean: route.description_korean.unwrap_or_else(|| "".to_string()),
                english: route.description_english.unwrap_or_else(|| "".to_string()),
            },
            stop_list,
        }
    }
}
