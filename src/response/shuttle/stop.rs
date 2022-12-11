use crate::model::shuttle::route_stop::ShuttleRouteStopItemWithDescription;
use crate::model::shuttle::stop::ShuttleStopItem;
use crate::model::shuttle::timetable::{
    ShuttleStopTimeTableItem, ShuttleTimeTableByShuttleStopItem,
};
use chrono::{Duration, NaiveTime};
use serde::Serialize;
use std::borrow::Borrow;
use std::collections::HashMap;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ShuttleStopListResponse {
    pub stop_list: Vec<ShuttleStopListItemResponse>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ShuttleStopListItemResponse {
    pub stop_name: String,
    pub location: Option<ShuttleStopLocationResponse>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ShuttleStopItemResponse {
    pub stop_name: String,
    pub location: Option<ShuttleStopLocationResponse>,
    pub route: Vec<ShuttleRouteStopResponse>,
}

#[derive(Serialize)]
pub struct ShuttleStopLocationResponse {
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ShuttleRouteStopResponse {
    pub name: String,
    pub description: ShuttleRouteDescriptionResponse,
    pub timetable: Vec<String>,
    pub arrival_list: Vec<i64>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ShuttleRouteStopTimetableResponse {
    pub timetable: Vec<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ShuttleRouteStopArrivalResponse {
    pub arrival_list: Vec<i64>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ShuttleRouteDescriptionResponse {
    pub korean: String,
    pub english: String,
}

impl ShuttleStopListResponse {
    pub fn new(stop_list: Vec<ShuttleStopItem>) -> Self {
        ShuttleStopListResponse {
            stop_list: stop_list
                .into_iter()
                .map(ShuttleStopListItemResponse::new)
                .collect(),
        }
    }
}

impl ShuttleStopListItemResponse {
    pub fn new(stop: ShuttleStopItem) -> Self {
        ShuttleStopListItemResponse {
            stop_name: stop.stop_name,
            location: ShuttleStopLocationResponse::new(stop.latitude, stop.longitude),
        }
    }
}

impl ShuttleStopItemResponse {
    pub fn new(
        stop_id: String,
        timetable: Vec<ShuttleStopTimeTableItem>,
        weekday: &bool,
        show_all: &Option<bool>,
    ) -> Self {
        let mut timetable_by_route: HashMap<(String, String, String), Vec<(bool, NaiveTime, i32)>> =
            HashMap::new();
        timetable.iter().for_each(|timetable| {
            let route_name = timetable.route_name.clone();
            let route_description_korean = timetable.route_description_korean.clone();
            let route_description_english = timetable.route_description_english.clone();
            let timetable_group = timetable_by_route
                .entry((
                    route_name,
                    route_description_korean.unwrap_or_default(),
                    route_description_english.unwrap_or_default(),
                ))
                .or_default();
            timetable_group.push((
                timetable.weekday,
                timetable.departure_time,
                timetable.cumulative_time.unwrap_or_default(),
            ));
        });

        ShuttleStopItemResponse {
            stop_name: stop_id,
            location: ShuttleStopLocationResponse::new(
                timetable[0].borrow().latitude,
                timetable[0].borrow().longitude,
            ),
            route: timetable_by_route
                .into_iter()
                .map(
                    |(
                        (route_name, route_description_korean, route_description_english),
                        timetable,
                    )| {
                        ShuttleRouteStopResponse::new(
                            route_name,
                            route_description_korean,
                            route_description_english,
                            timetable,
                            weekday,
                            show_all,
                        )
                    },
                )
                .collect(),
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

impl ShuttleRouteStopResponse {
    pub fn new(
        route_name: String,
        route_description_korean: String,
        route_description_english: String,
        timetable: Vec<(bool, NaiveTime, i32)>,
        weekday: &bool,
        show_all: &Option<bool>,
    ) -> Self {
        let now = chrono::Local::now().time();
        ShuttleRouteStopResponse {
            name: route_name,
            description: ShuttleRouteDescriptionResponse {
                korean: route_description_korean,
                english: route_description_english,
            },
            timetable: timetable
                .iter()
                .filter(|(timetable_weekday, departure_time, cumulative_time)| {
                    timetable_weekday == weekday
                        && (*departure_time + Duration::minutes(*cumulative_time as i64) >= now
                            || show_all.unwrap_or(false))
                })
                .map(|(_, departure_time, cumulative_time)| {
                    (*departure_time + Duration::minutes(*cumulative_time as i64)).to_string()
                })
                .collect(),
            arrival_list: timetable
                .iter()
                .filter(|(timetable_weekday, departure_time, cumulative_time)| {
                    timetable_weekday == weekday
                        && (*departure_time + Duration::minutes(*cumulative_time as i64) >= now)
                })
                .map(|(_, departure_time, cumulative_time)| {
                    (*departure_time + Duration::minutes(*cumulative_time as i64) - now)
                        .num_minutes()
                })
                .collect(),
        }
    }
}

impl ShuttleRouteStopTimetableResponse {
    pub fn new(
        route: &ShuttleRouteStopItemWithDescription,
        timetable_list: &Vec<&ShuttleTimeTableByShuttleStopItem>,
    ) -> Self {
        ShuttleRouteStopTimetableResponse {
            timetable: timetable_list
                .iter()
                .map(|item| {
                    (item.departure_time + Duration::minutes(route.cumulative_time.unwrap() as i64))
                        .to_string()
                })
                .collect(),
        }
    }
}

impl ShuttleRouteStopArrivalResponse {
    pub fn new(
        route: &ShuttleRouteStopItemWithDescription,
        timetable_list: Vec<ShuttleTimeTableByShuttleStopItem>,
    ) -> Self {
        let now = chrono::Local::now().time();
        ShuttleRouteStopArrivalResponse {
            arrival_list: timetable_list
                .iter()
                .map(|item| {
                    (item.departure_time + Duration::minutes(route.cumulative_time.unwrap() as i64)
                        - now)
                        .num_minutes()
                })
                .collect(),
        }
    }
}
