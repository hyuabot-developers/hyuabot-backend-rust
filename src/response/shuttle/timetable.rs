use chrono::Duration;
use serde::Serialize;
use std::collections::HashMap;

use crate::model::shuttle::timetable::EntireShuttleTimeTableItem;
use crate::utils::shuttle::get_shuttle_weekday;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ShuttleTimetableListResponse {
    pub stop: Vec<ShuttleTimetableStopItem>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ShuttleArrivalListResponse {
    pub stop: Vec<ShuttleArrivalStopItem>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ShuttleTimetableStopItem {
    pub name: String,
    pub route: Vec<ShuttleTimetableRouteStopItem>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ShuttleArrivalStopItem {
    pub name: String,
    pub route: Vec<ShuttleArrivalRouteStopItem>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ShuttleTimetableRouteStopItem {
    pub name: String,
    pub weekdays: Vec<String>,
    pub weekends: Vec<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ShuttleArrivalRouteStopItem {
    pub name: String,
    pub arrival: Vec<i64>,
}

impl ShuttleTimetableListResponse {
    pub fn new(timetable_list: &[EntireShuttleTimeTableItem]) -> Self {
        let mut timetable_groups: HashMap<
            String,
            HashMap<String, Vec<&EntireShuttleTimeTableItem>>,
        > = HashMap::new();
        timetable_list.into_iter().for_each(|timetable| {
            let stop_name = timetable.stop_name.clone();
            let route_name = timetable.route_name.clone();
            let timetable_group = timetable_groups
                .entry(stop_name)
                .or_default()
                .entry(route_name)
                .or_default();
            timetable_group.push(timetable);
        });
        ShuttleTimetableListResponse {
            stop: timetable_groups
                .into_iter()
                .map(|(stop_name, timetable_group)| {
                    ShuttleTimetableStopItem::new(stop_name, timetable_group)
                })
                .collect(),
        }
    }
}

impl ShuttleTimetableStopItem {
    pub fn new(
        stop_name: String,
        timetable_group: HashMap<String, Vec<&EntireShuttleTimeTableItem>>,
    ) -> Self {
        ShuttleTimetableStopItem {
            name: stop_name,
            route: timetable_group
                .into_iter()
                .map(|(route_name, timetable_list)| {
                    ShuttleTimetableRouteStopItem::new(route_name, timetable_list)
                })
                .collect(),
        }
    }
}

impl ShuttleTimetableRouteStopItem {
    pub fn new(route_name: String, timetable_list: Vec<&EntireShuttleTimeTableItem>) -> Self {
        ShuttleTimetableRouteStopItem {
            name: route_name.clone(),
            weekdays: timetable_list
                .iter()
                .filter(|timetable| timetable.weekday)
                .map(|timetable| {
                    (timetable.departure_time
                        + Duration::minutes(timetable.cumulative_time.unwrap_or_default() as i64))
                    .clone()
                    .to_string()
                })
                .collect(),
            weekends: timetable_list
                .iter()
                .filter(|timetable| !timetable.weekday)
                .map(|timetable| {
                    (timetable.departure_time
                        + Duration::minutes(timetable.cumulative_time.unwrap_or_default() as i64))
                    .clone()
                    .to_string()
                })
                .collect(),
        }
    }
}

impl ShuttleArrivalListResponse {
    pub fn new(timetable_list: &[EntireShuttleTimeTableItem]) -> Self {
        let mut timetable_groups: HashMap<
            String,
            HashMap<String, Vec<&EntireShuttleTimeTableItem>>,
        > = HashMap::new();
        timetable_list.into_iter().for_each(|timetable| {
            let stop_name = timetable.stop_name.clone();
            let route_name = timetable.route_name.clone();
            let timetable_group = timetable_groups
                .entry(stop_name)
                .or_default()
                .entry(route_name)
                .or_default();
            timetable_group.push(timetable);
        });
        let weekday = get_shuttle_weekday() == "weekdays";
        ShuttleArrivalListResponse {
            stop: timetable_groups
                .into_iter()
                .map(|(stop_name, timetable_group)| {
                    ShuttleArrivalStopItem::new(stop_name, weekday, timetable_group)
                })
                .collect(),
        }
    }
}

impl ShuttleArrivalStopItem {
    pub fn new(
        stop_name: String,
        weekday: bool,
        timetable_group: HashMap<String, Vec<&EntireShuttleTimeTableItem>>,
    ) -> Self {
        ShuttleArrivalStopItem {
            name: stop_name,
            route: timetable_group
                .into_iter()
                .map(|(route_name, timetable_list)| {
                    ShuttleArrivalRouteStopItem::new(route_name.clone(), weekday, timetable_list)
                })
                .collect(),
        }
    }
}

impl ShuttleArrivalRouteStopItem {
    pub fn new(
        route_name: String,
        weekday: bool,
        timetable: Vec<&EntireShuttleTimeTableItem>,
    ) -> Self {
        let now = chrono::Local::now().naive_local().time();
        ShuttleArrivalRouteStopItem {
            name: route_name.clone(),
            arrival: timetable
                .iter()
                .filter(|timetable| timetable.weekday == weekday)
                .map(|item| {
                    (item.departure_time + Duration::minutes(item.cumulative_time.unwrap() as i64)
                        - now)
                        .num_minutes()
                })
                .collect(),
        }
    }
}
