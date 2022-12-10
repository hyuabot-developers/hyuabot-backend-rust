use std::ops::Add;
use chrono::Duration;
use serde::Serialize;
use crate::model::shuttle::route::ShuttleRouteItem;
use crate::model::shuttle::route_stop::ShuttleRouteStopItem;
use crate::model::shuttle::timetable::ShuttleTimeTableItem;

#[derive(Serialize)]
pub struct ShuttleRouteListResponse {
    pub routes: Vec<ShuttleRouteListItem>,
}

#[derive(Serialize)]
pub struct ShuttleRouteListItem {
    pub name: String,
    pub description: ShuttleDescriptionItem,
}


#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ShuttleRouteResponse {
    pub name: String,
    pub description: ShuttleDescriptionItem,
    pub stop_list: Vec<ShuttleRouteStopItemResponse>,
    pub location_list: Vec<ShuttleLocationItem>
}

#[derive(Serialize)]
pub struct ShuttleDescriptionItem {
    pub korean: String,
    pub english: String,
}

#[derive(Serialize)]
pub struct ShuttleRouteStopItemResponse {
    pub name: String,
    pub order: i32,
    pub weekdays: ShuttleFirstLastTimeItem,
    pub weekends: ShuttleFirstLastTimeItem,
}

#[derive(Serialize)]
pub struct ShuttleFirstLastTimeItem {
    pub first: String,
    pub last: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ShuttleLocationResponse {
    pub location_list: Vec<ShuttleLocationItem>
}

#[derive(Serialize)]
pub struct ShuttleLocationItem {
    pub location: f32,
}


impl ShuttleRouteListResponse {
    pub fn new(routes: Vec<ShuttleRouteItem>) -> Self {
        ShuttleRouteListResponse {
            routes: routes.into_iter().map(ShuttleRouteListItem::new).collect()
        }
    }
}

impl ShuttleRouteListItem {
    pub fn new(route: ShuttleRouteItem) -> Self {
        ShuttleRouteListItem {
            name: route.route_name,
            description: ShuttleDescriptionItem {
                korean: route.description_korean.unwrap_or_default(),
                english: route.description_english.unwrap_or_default(),
            }
        }
    }
}

impl ShuttleRouteResponse {
    pub fn new(route: ShuttleRouteItem, weekday: &str, stop_items: &[ShuttleRouteStopItem], timetable: &[ShuttleTimeTableItem]) -> Self {
        let weekdays_shuttle = timetable.iter().filter(|item| item.weekday).collect::<Vec<&ShuttleTimeTableItem>>();
        let weekends_shuttle = timetable.iter().filter(|item| !item.weekday).collect::<Vec<&ShuttleTimeTableItem>>();
        let first_cumulative_time = match stop_items.first() {
            Some(item) => -item.cumulative_time.unwrap(),
            None => 0,
        };
        let last_cumulative_time = match stop_items.last() {
            Some(item) => -item.cumulative_time.unwrap(),
            None => 0,
        };
        let running_shuttle = match weekday {
            "halt" => Vec::new(),
            _ => timetable.iter().filter(|item|
                item.weekday == (weekday == "weekdays") &&
                    item.departure_time <= chrono::Local::now().time().add(Duration::minutes(first_cumulative_time as i64)) &&
                    item.departure_time >= chrono::Local::now().time().add(Duration::minutes(last_cumulative_time as i64))
            ).collect::<Vec<&ShuttleTimeTableItem>>(),
        };
        let mut stop_list = Vec::new();
        let _ = stop_items.iter().map(
            |stop_item| {
                stop_list.push(ShuttleRouteStopItemResponse {
                    name: stop_item.stop_name.clone(),
                    order: stop_item.stop_order.unwrap(),
                    weekdays: ShuttleFirstLastTimeItem::new(
                        weekdays_shuttle.first(), weekdays_shuttle.last(), stop_item.cumulative_time.unwrap() as i64
                    ),
                    weekends: ShuttleFirstLastTimeItem::new(
                        weekends_shuttle.first(), weekends_shuttle.last(), stop_item.cumulative_time.unwrap() as i64
                    ),
                });
            }
        ).collect::<Vec<()>>();
        let mut location = Vec::new();
        let _ = running_shuttle.iter()
            .map(|item| {
                let departed_before = (chrono::Local::now().time() - item.departure_time).num_minutes() as i32;
                let current = stop_items.iter().find(|stop_item| stop_item.cumulative_time.unwrap() >= departed_before).unwrap();
                location.push(ShuttleLocationItem {
                    location: if current.cumulative_time.unwrap() == departed_before || current.stop_order.unwrap() == 0 {
                        current.stop_order.unwrap() as f32
                    } else {
                        let previous_cumulative_time = stop_items[(current.stop_order.unwrap() - 1) as usize].cumulative_time.unwrap();
                        (departed_before as f32 - previous_cumulative_time as f32) / (current.cumulative_time.unwrap() - previous_cumulative_time) as f32 + (current.stop_order.unwrap() - 1) as f32
                    },
                });
            }).collect::<Vec<()>>();
        location.sort_by(|a, b| a.location.partial_cmp(&b.location).unwrap());
        ShuttleRouteResponse {
            name: route.route_name,
            description: ShuttleDescriptionItem {
                korean: route.description_korean.unwrap_or_default(),
                english: route.description_english.unwrap_or_default(),
            },
            stop_list,
            location_list: location,
        }
    }
}

impl ShuttleLocationResponse {
    pub fn new(weekday: &str, stop_items: &[ShuttleRouteStopItem], timetable: &[ShuttleTimeTableItem]) -> Self {
        let first_cumulative_time = match stop_items.first() {
            Some(item) => - item.cumulative_time.unwrap(),
            None => 0,
        };
        let last_cumulative_time = match stop_items.last() {
            Some(item) => - item.cumulative_time.unwrap(),
            None => 0,
        };
        let running_shuttle = match weekday {
            "halt" => Vec::new(),
            _ => timetable.iter().filter(|item|
                item.weekday == (weekday == "weekdays") &&
                    item.departure_time <= chrono::Local::now().time().add(Duration::minutes(first_cumulative_time as i64)) &&
                    item.departure_time >= chrono::Local::now().time().add(Duration::minutes(last_cumulative_time as i64))
            ).collect::<Vec<&ShuttleTimeTableItem>>(),
        };
        let mut location = Vec::new();
        let _ = running_shuttle.iter()
            .map(|item| {
                let departed_before = (chrono::Local::now().time() - item.departure_time).num_minutes() as i32;
                let current = stop_items.iter().find(|stop_item| stop_item.cumulative_time.unwrap() >= departed_before).unwrap();
                location.push(ShuttleLocationItem {
                    location: if current.cumulative_time.unwrap() == departed_before || current.stop_order.unwrap() == 0 {
                        current.stop_order.unwrap() as f32
                    } else {
                        let previous_cumulative_time = stop_items[(current.stop_order.unwrap() - 1) as usize].cumulative_time.unwrap();
                        (departed_before as f32 - previous_cumulative_time as f32) / (current.cumulative_time.unwrap() - previous_cumulative_time) as f32 + (current.stop_order.unwrap() - 1) as f32
                    },
                });
            }).collect::<Vec<()>>();
        location.sort_by(|a, b| a.location.partial_cmp(&b.location).unwrap());
        ShuttleLocationResponse {
            location_list: location,
        }
    }
}

impl ShuttleFirstLastTimeItem {
    pub fn new(first: Option<&&ShuttleTimeTableItem>, last: Option<&&ShuttleTimeTableItem>, cumulative_time: i64) -> Self {
        ShuttleFirstLastTimeItem {
            first: match first {
                Some(item) => item.departure_time.add(Duration::minutes(cumulative_time)).to_string(),
                None => "00:00:00".to_string(),
            },
            last: match last {
                Some(item) => item.departure_time.add(Duration::minutes(cumulative_time)).to_string(),
                None => "00:00:00".to_string(),
            },
        }
    }
}