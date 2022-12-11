use crate::model::bus::route::BusRouteItem;
use crate::model::bus::stop::BusStopItem;
use crate::model::bus::timetable::BusTimetableItem;
use chrono::NaiveTime;
use serde::Serialize;

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
    pub route_type: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BusRouteItemResponse {
    pub route_id: i32,
    pub route_name: String,
    pub route_type: BusRouteType,
    pub company: BusRouteItemCompany,
    pub running_time: BusRouteItemRunningTime,
    pub from: BusRouteStop,
    pub to: BusRouteStop,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BusRouteItemCompany {
    pub id: i32,
    pub name: String,
    pub telephone: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BusRouteItemRunningTime {
    pub up: BusRouteItemFirstLastTime,
    pub down: BusRouteItemFirstLastTime,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BusRouteItemFirstLastTime {
    pub first: String,
    pub last: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BusRouteType {
    pub code: String,
    pub name: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BusRouteStop {
    pub id: i32,
    pub name: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BusTimetableListResponse {
    pub weekdays: Vec<String>,
    pub saturday: Vec<String>,
    pub sunday: Vec<String>,
}

impl BusRouteListResponse {
    pub fn new(route_list: Vec<BusRouteItem>) -> Self {
        BusRouteListResponse {
            route_list: route_list.into_iter().map(BusRouteListItem::new).collect(),
        }
    }
}

impl BusRouteListItem {
    pub fn new(route_item: BusRouteItem) -> Self {
        BusRouteListItem {
            route_id: route_item.route_id,
            route_name: route_item.route_name,
            route_type: route_item.route_type_name,
        }
    }
}

impl BusRouteItemResponse {
    pub fn new(route_item: BusRouteItem) -> Self {
        BusRouteItemResponse {
            route_id: route_item.route_id,
            route_name: route_item.route_name,
            route_type: BusRouteType::new(route_item.route_type_code, route_item.route_type_name),
            company: BusRouteItemCompany::new(
                route_item.company_id.unwrap(),
                route_item.company_name,
                route_item.company_telephone,
            ),
            running_time: BusRouteItemRunningTime::new(
                route_item.up_first_time,
                route_item.up_last_time,
                route_item.down_first_time,
                route_item.down_last_time,
            ),
            from: BusRouteStop::new(route_item.start_stop_id),
            to: BusRouteStop::new(route_item.end_stop_id),
        }
    }
}

impl BusRouteItemCompany {
    pub fn new(id: i32, name: String, telephone: String) -> Self {
        BusRouteItemCompany {
            id,
            name,
            telephone,
        }
    }
}

impl BusRouteItemRunningTime {
    pub fn new(
        up_first_time: NaiveTime,
        up_last_time: NaiveTime,
        down_first_time: NaiveTime,
        down_last_time: NaiveTime,
    ) -> Self {
        BusRouteItemRunningTime {
            up: BusRouteItemFirstLastTime::new(up_first_time, up_last_time),
            down: BusRouteItemFirstLastTime::new(down_first_time, down_last_time),
        }
    }
}

impl BusRouteItemFirstLastTime {
    pub fn new(first: NaiveTime, last: NaiveTime) -> Self {
        BusRouteItemFirstLastTime {
            first: first.to_string(),
            last: last.to_string(),
        }
    }
}

impl BusRouteType {
    pub fn new(code: String, name: String) -> Self {
        BusRouteType { code, name }
    }
}

impl BusRouteStop {
    pub fn new(stop_id: i32) -> Self {
        let stop_item = BusStopItem::get_one_by_id(&stop_id).unwrap();
        BusRouteStop {
            id: stop_item.stop_id,
            name: stop_item.stop_name.unwrap(),
        }
    }
}

impl BusTimetableListResponse {
    pub fn new(timetable: Vec<BusTimetableItem>) -> Self {
        BusTimetableListResponse {
            weekdays: timetable
                .iter()
                .filter(|item| item.weekday == "weekdays")
                .map(|item| item.departure_time.to_string())
                .collect(),
            saturday: timetable
                .iter()
                .filter(|item| item.weekday == "saturday")
                .map(|item| item.departure_time.to_string())
                .collect(),
            sunday: timetable
                .iter()
                .filter(|item| item.weekday == "sunday")
                .map(|item| item.departure_time.to_string())
                .collect(),
        }
    }
}
