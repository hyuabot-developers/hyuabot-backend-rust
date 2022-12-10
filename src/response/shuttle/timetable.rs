use chrono::{Datelike, Duration};
use serde::Serialize;
use crate::model::shuttle::period::{ShuttleHolidayItem, ShuttlePeriodItem};

use crate::model::shuttle::route_stop::{ShuttleRouteStopItem, ShuttleRouteStopItemWithDescription};
use crate::model::shuttle::stop::ShuttleStopItem;
use crate::model::shuttle::timetable::ShuttleTimeTableByShuttleStopItem;
use crate::response::shuttle::stop::ShuttleRouteDescriptionResponse;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ShuttleTimetableListResponse {
    pub stop_list: Vec<ShuttleTimetableStopItem>,
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
    pub route_list: Vec<ShuttleTimetableRouteStopItem>,
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
    pub description: ShuttleRouteDescriptionResponse,
    pub timetable: Vec<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ShuttleArrivalRouteStopItem {
    pub name: String,
    pub description: ShuttleRouteDescriptionResponse,
    pub arrival: Vec<i64>,
}

impl ShuttleTimetableListResponse {
    pub fn new(stop_list: &[ShuttleStopItem]) -> Self {
        let period = ShuttlePeriodItem::get_current_period().unwrap();
        let weekday = match ShuttleHolidayItem::get_holiday_by_date(chrono::Local::now().naive_local()) {
            Ok(holiday_item) => holiday_item.holiday_type,
            Err(_) => {
                if chrono::Local::now().weekday() == chrono::Weekday::Sat || chrono::Local::now().weekday() == chrono::Weekday::Sun {
                    "weekends".to_string()
                } else {
                    "weekdays".to_string()
                }
            }
        };
        ShuttleTimetableListResponse{
            stop_list: stop_list.iter().map(|stop| ShuttleTimetableStopItem::new(stop, &period.period_type, &(weekday == "weekdays"))).collect()
        }
    }
}

impl ShuttleTimetableStopItem {
    pub fn new(stop: &ShuttleStopItem, period: &str, weekday: &bool) -> Self {
        let route_list = ShuttleRouteStopItem::get_route_list_by_stop_name(&stop.stop_name)
            .unwrap_or_else(|_| vec![]);
        ShuttleTimetableStopItem {
            name: stop.stop_name.clone(),
            route_list: route_list.iter().map(| route| ShuttleTimetableRouteStopItem::new(route, period, weekday)).collect()
        }
    }
}

impl ShuttleTimetableRouteStopItem {
    pub fn new(route_stop: &ShuttleRouteStopItemWithDescription, period: &str, weekday: &bool) -> Self {
        let timetable = ShuttleTimeTableByShuttleStopItem::get_timetable_by_route_stop_name(
            period, weekday, route_stop, &999, &Option::from(true)
        ).unwrap();
        ShuttleTimetableRouteStopItem {
            name: route_stop.route_name.clone(),
            description: ShuttleRouteDescriptionResponse {
                korean: route_stop.description_korean.clone().unwrap(),
                english: route_stop.description_english.clone().unwrap()
            },
            timetable: timetable.iter()
                .map(|item| (item.departure_time + Duration::minutes(route_stop.cumulative_time.unwrap() as i64)).to_string())
                .collect()
        }
    }
}

impl ShuttleArrivalListResponse {
    pub fn new(stop_list: &[ShuttleStopItem]) -> Self {
        let period = ShuttlePeriodItem::get_current_period().unwrap();
        let weekday = match ShuttleHolidayItem::get_holiday_by_date(chrono::Local::now().naive_local()) {
            Ok(holiday_item) => holiday_item.holiday_type,
            Err(_) => {
                if chrono::Local::now().weekday() == chrono::Weekday::Sat || chrono::Local::now().weekday() == chrono::Weekday::Sun {
                    "weekends".to_string()
                } else {
                    "weekdays".to_string()
                }
            }
        };
        ShuttleArrivalListResponse{
            stop_list: stop_list.iter().map(|stop| ShuttleArrivalStopItem::new(stop, &period.period_type, &(weekday == "weekdays"))).collect()
        }
    }
}

impl ShuttleArrivalStopItem {
    pub fn new(stop: &ShuttleStopItem, period: &str, weekday: &bool) -> Self {
        let route_list = ShuttleRouteStopItem::get_route_list_by_stop_name(&stop.stop_name)
            .unwrap_or_else(|_| vec![]);
        ShuttleArrivalStopItem {
            name: stop.stop_name.clone(),
            route_list: route_list.iter().map(| route| ShuttleArrivalRouteStopItem::new(route, period, weekday)).collect()
        }
    }
}

impl ShuttleArrivalRouteStopItem {
    pub fn new(route_stop: &ShuttleRouteStopItemWithDescription, period: &str, weekday: &bool) -> Self {
        let timetable = ShuttleTimeTableByShuttleStopItem::get_timetable_by_route_stop_name(
            period, weekday, route_stop, &999, &Option::from(false)
        ).unwrap();
        let now = chrono::Local::now().naive_local().time();
        ShuttleArrivalRouteStopItem {
            name: route_stop.route_name.clone(),
            description: ShuttleRouteDescriptionResponse {
                korean: route_stop.description_korean.clone().unwrap(),
                english: route_stop.description_english.clone().unwrap()
            },
            arrival: timetable.iter()
                .map(|item| (item.departure_time + Duration::minutes(route_stop.cumulative_time.unwrap() as i64) - now).num_minutes())
                .collect()
        }
    }
}