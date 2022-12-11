use crate::model::bus::realtime::BusRealtimeItem;
use crate::model::bus::route_stop::BusRouteStopItem;
use crate::model::bus::stop::BusStopItem;
use crate::model::bus::timetable::BusTimetableItem;
use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BusStopListResponse {
    pub stop: Vec<BusStopListItem>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BusStopListItem {
    pub id: i32,
    pub name: String,
    pub location: BusStopLocation,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BusStopItemResponse {
    pub id: i32,
    pub name: String,
    pub district_code: i32,
    pub mobile_number: String,
    pub region_name: String,
    pub location: BusStopLocation,
    pub route_list: Vec<BusViaRouteItem>,
}

#[derive(Serialize)]
pub struct BusStopLocation {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BusViaRouteItem {
    id: i32,
    name: String,
    sequence: i32,
    realtime: Vec<BusViaRouteRealtimeItem>,
    timetable: BusViaRouteTimetableList,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BusViaRouteRealtimeItem {
    stop: i32,
    seat: i32,
    time: i64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BusViaRouteTimetableList {
    start_stop: BusViaRouteTimetableStartStop,
    departure_time: Vec<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BusViaRouteTimetableStartStop {
    name: String,
    id: i32,
}

impl BusStopListResponse {
    pub fn new(stop_list: Vec<BusStopItem>) -> Self {
        Self {
            stop: stop_list.into_iter().map(BusStopListItem::new).collect(),
        }
    }
}

impl BusStopListItem {
    pub fn new(stop_item: BusStopItem) -> Self {
        Self {
            id: stop_item.stop_id,
            name: stop_item.stop_name.unwrap(),
            location: BusStopLocation::new(stop_item.latitude, stop_item.longitude),
        }
    }
}

impl BusStopLocation {
    pub fn new(latitude: f64, longitude: f64) -> Self {
        Self {
            latitude,
            longitude,
        }
    }
}

impl BusStopItemResponse {
    pub fn new(stop_item: BusStopItem) -> Self {
        let via_route_list = BusRouteStopItem::find_by_stop_id(&stop_item.stop_id).unwrap();
        Self {
            id: stop_item.stop_id,
            name: stop_item.stop_name.unwrap(),
            district_code: stop_item.district_code,
            mobile_number: stop_item.mobile_number,
            region_name: stop_item.region_name,
            location: BusStopLocation::new(stop_item.latitude, stop_item.longitude),
            route_list: via_route_list
                .into_iter()
                .map(BusViaRouteItem::new)
                .collect(),
        }
    }
}

impl BusViaRouteItem {
    pub fn new(route_stop_item: BusRouteStopItem) -> Self {
        let realtime_arrival_list = BusRealtimeItem::find_by_stop_and_route_id(
            &route_stop_item.route_id,
            &route_stop_item.stop_id,
        )
        .unwrap();
        let now = chrono::Local::now().naive_local();
        Self {
            id: route_stop_item.route_id,
            name: route_stop_item.route_name.clone(),
            sequence: route_stop_item.stop_sequence,
            realtime: realtime_arrival_list
                .into_iter()
                .filter(|realtime_item| {
                    realtime_item.remaining_time as i64
                        - (now - realtime_item.last_updated_at).num_minutes()
                        > 0
                })
                .map(|realtime_item| BusViaRouteRealtimeItem::new(realtime_item, now))
                .collect(),
            timetable: BusViaRouteTimetableList::new(&route_stop_item),
        }
    }
}

impl BusViaRouteRealtimeItem {
    pub fn new(realtime_item: BusRealtimeItem, now: NaiveDateTime) -> Self {
        Self {
            stop: realtime_item.remaining_stop_count,
            seat: realtime_item.remaining_seat_count,
            time: realtime_item.remaining_time as i64
                - (now - realtime_item.last_updated_at).num_minutes(),
        }
    }
}

impl BusViaRouteTimetableList {
    pub fn new(route_stop_item: &BusRouteStopItem) -> Self {
        let timetable = BusTimetableItem::find_by_weekday_stop_and_route_id(
            &route_stop_item.route_id,
            &route_stop_item.start_stop_id,
            None,
            None,
        )
        .unwrap();
        Self {
            start_stop: BusViaRouteTimetableStartStop::new(
                BusStopItem::get_one_by_id(&route_stop_item.start_stop_id).unwrap(),
            ),
            departure_time: timetable
                .into_iter()
                .map(|timetable_item| timetable_item.departure_time.to_string())
                .collect(),
        }
    }
}

impl BusViaRouteTimetableStartStop {
    pub fn new(stop_item: BusStopItem) -> Self {
        Self {
            name: stop_item.stop_name.unwrap(),
            id: stop_item.stop_id,
        }
    }
}
