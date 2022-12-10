use crate::model::shuttle::period::ShuttlePeriodItem;
use crate::model::shuttle::route_stop::ShuttleRouteStopItemWithDescription;
use crate::model::shuttle::stop::ShuttleStopItem;
use crate::model::shuttle::timetable::ShuttleTimeTableByShuttleStopItem;
use chrono::Duration;
use serde::Serialize;

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
    pub route_list: Vec<ShuttleRouteStopResponse>,
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
        stop_item: ShuttleStopItem,
        _routes: &Vec<ShuttleRouteStopItemWithDescription>,
        _period: &ShuttlePeriodItem,
        _weekday: &bool,
        _limit: &i64,
        _show_all: &Option<bool>,
    ) -> Self {
        let route_list = Vec::new();
        ShuttleStopItemResponse {
            stop_name: stop_item.stop_name,
            location: ShuttleStopLocationResponse::new(stop_item.latitude, stop_item.longitude),
            route_list,
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
        route: &ShuttleRouteStopItemWithDescription,
        timetable_list: Vec<ShuttleTimeTableByShuttleStopItem>,
    ) -> Self {
        let description_korean = route.description_korean.clone().unwrap_or_default();
        let description_english = route.description_english.clone().unwrap_or_default();
        let now = chrono::Local::now().time();
        ShuttleRouteStopResponse {
            name: route.route_name.clone(),
            description: ShuttleRouteDescriptionResponse {
                korean: description_korean,
                english: description_english,
            },
            timetable: timetable_list
                .iter()
                .map(|item| {
                    (item.departure_time + Duration::minutes(route.cumulative_time.unwrap() as i64))
                        .to_string()
                })
                .collect(),
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
