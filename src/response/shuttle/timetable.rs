use crate::model::shuttle::period::{ShuttleHolidayItem, ShuttlePeriodItem};
use chrono::{Datelike, Duration};
use serde::Serialize;
use std::collections::HashMap;

use crate::model::shuttle::route_stop::{
    ShuttleRouteStopItem, ShuttleRouteStopItemWithDescription,
};
use crate::model::shuttle::stop::ShuttleStopItem;
use crate::model::shuttle::timetable::{
    EntireShuttleTimeTableItem, ShuttleTimeTableByShuttleStopItem,
};
use crate::response::shuttle::stop::ShuttleRouteDescriptionResponse;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ShuttleTimetableListResponse {
    pub stop: Vec<ShuttleTimetableStopItem>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ShuttleArrivalListResponse {
    pub stop_list: Vec<ShuttleArrivalStopItem>,
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
    pub route_list: Vec<ShuttleArrivalRouteStopItem>,
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
    pub description: ShuttleRouteDescriptionResponse,
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
    pub fn new(stop_list: &[ShuttleStopItem]) -> Self {
        let period = ShuttlePeriodItem::get_current_period().unwrap();
        let weekday =
            match ShuttleHolidayItem::get_holiday_by_date(chrono::Local::now().naive_local()) {
                Ok(holiday_item) => holiday_item.holiday_type,
                Err(_) => {
                    if chrono::Local::now().weekday() == chrono::Weekday::Sat
                        || chrono::Local::now().weekday() == chrono::Weekday::Sun
                    {
                        "weekends".to_string()
                    } else {
                        "weekdays".to_string()
                    }
                }
            };
        ShuttleArrivalListResponse {
            stop_list: stop_list
                .iter()
                .map(|stop| {
                    ShuttleArrivalStopItem::new(stop, &period.period_type, &(weekday == "weekdays"))
                })
                .collect(),
        }
    }
}

impl ShuttleArrivalStopItem {
    pub fn new(stop: &ShuttleStopItem, period: &str, weekday: &bool) -> Self {
        let route_list = ShuttleRouteStopItem::get_route_list_by_stop_name(&stop.stop_name)
            .unwrap_or_else(|_| vec![]);
        ShuttleArrivalStopItem {
            name: stop.stop_name.clone(),
            route_list: route_list
                .iter()
                .map(|route| ShuttleArrivalRouteStopItem::new(route, period, weekday))
                .collect(),
        }
    }
}

impl ShuttleArrivalRouteStopItem {
    pub fn new(
        route_stop: &ShuttleRouteStopItemWithDescription,
        period: &str,
        weekday: &bool,
    ) -> Self {
        let timetable = ShuttleTimeTableByShuttleStopItem::get_timetable_by_route_stop_name(
            period,
            weekday,
            route_stop,
            &999,
            &Option::from(false),
        )
        .unwrap();
        let now = chrono::Local::now().naive_local().time();
        ShuttleArrivalRouteStopItem {
            name: route_stop.route_name.clone(),
            description: ShuttleRouteDescriptionResponse {
                korean: route_stop.description_korean.clone().unwrap(),
                english: route_stop.description_english.clone().unwrap(),
            },
            arrival: timetable
                .iter()
                .map(|item| {
                    (item.departure_time
                        + Duration::minutes(route_stop.cumulative_time.unwrap() as i64)
                        - now)
                        .num_minutes()
                })
                .collect(),
        }
    }
}
