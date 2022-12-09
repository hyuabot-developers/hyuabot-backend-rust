use chrono::NaiveTime;
use serde::Serialize;
use crate::model::subway::station::SubwayStationItem;
use crate::model::subway::timetable::SubwayTimetableItem;
use crate::utils::subway::get_subway_weekday;


#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubwayStationListResponse {
    pub station_list: Vec<SubwayStationListItem>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubwayStationListItem {
    pub station_id: String,
    pub route_id: i32,
    pub station_name: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubwayStationItemResponse {
    pub station_id: String,
    pub route_id: i32,
    pub station_name: String,
    pub station_sequence: i32,
    pub running_time: SubwayStationItemRunningTime,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubwayStationItemRunningTime {
    pub up: SubwayStationItemFirstLastTime,
    pub down: SubwayStationItemFirstLastTime
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubwayStationItemFirstLastTime {
    pub first: SubwayTimeItem,
    pub last: SubwayTimeItem
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubwayTimeItem {
    pub terminal_station: String,
    pub time: String,
}

impl SubwayStationListResponse {
    pub fn new(station_list: Vec<SubwayStationItem>) -> Self {
        Self {
            station_list: station_list.into_iter().map(|stop_item| SubwayStationListItem::new(stop_item)).collect()
        }
    }
}

impl SubwayStationListItem {
    pub fn new(station_item: SubwayStationItem) -> Self {
        Self {
            station_id: station_item.station_id,
            route_id: station_item.route_id,
            station_name: station_item.station_name,
        }
    }
}

impl SubwayStationItemResponse {
    pub fn new(station_item: SubwayStationItem) -> Self {
        let up_first_train = SubwayTimetableItem::get_first_train_by_heading(&station_item.station_id, &get_subway_weekday(), "up")
            .unwrap_or_else(
                |_| {
                    SubwayTimetableItem {
                        station_id: String::from(""),
                        terminal_station_id: String::from(""),
                        departure_time: NaiveTime::parse_from_str("00:00:00", "%H:%M:%S").unwrap(),
                        weekday: String::from(""),
                        up_down_type: String::from(""),
                    }
                }
            );
        let down_first_train = SubwayTimetableItem::get_first_train_by_heading(&station_item.station_id, &get_subway_weekday(), "down")
            .unwrap_or_else(
                |_| {
                    SubwayTimetableItem {
                        station_id: String::from(""),
                        terminal_station_id: String::from(""),
                        departure_time: NaiveTime::parse_from_str("00:00:00", "%H:%M:%S").unwrap(),
                        weekday: String::from(""),
                        up_down_type: String::from(""),
                    }
                }
            );
        let up_last_train = SubwayTimetableItem::get_last_train_by_heading(&station_item.station_id, &get_subway_weekday(), "up")
            .unwrap_or_else(
                |_| {
                    SubwayTimetableItem {
                        station_id: String::from(""),
                        terminal_station_id: String::from(""),
                        departure_time: NaiveTime::parse_from_str("00:00:00", "%H:%M:%S").unwrap(),
                        weekday: String::from(""),
                        up_down_type: String::from(""),
                    }
                }
            );
        let down_last_train = SubwayTimetableItem::get_last_train_by_heading(&station_item.station_id, &get_subway_weekday(), "down")
            .unwrap_or_else(
                |_| {
                    SubwayTimetableItem {
                        station_id: String::from(""),
                        terminal_station_id: String::from(""),
                        departure_time: NaiveTime::parse_from_str("00:00:00", "%H:%M:%S").unwrap(),
                        weekday: String::from(""),
                        up_down_type: String::from(""),
                    }
                }
            );
        Self {
            station_id: station_item.station_id.clone(),
            route_id: station_item.route_id,
            station_name: station_item.station_name,
            station_sequence: station_item.station_sequence,
            running_time: SubwayStationItemRunningTime::new(
                up_first_train, up_last_train, down_first_train, down_last_train
            ),
        }
    }
}

impl SubwayStationItemRunningTime {
    pub fn new(
        up_first_time: SubwayTimetableItem,
        up_last_time: SubwayTimetableItem,
        down_first_time: SubwayTimetableItem,
        down_last_time: SubwayTimetableItem,
    ) -> Self {
        Self {
            up: SubwayStationItemFirstLastTime::new(up_first_time, up_last_time),
            down: SubwayStationItemFirstLastTime::new(down_first_time, down_last_time),
        }
    }
}

impl SubwayStationItemFirstLastTime {
    pub fn new(first_time: SubwayTimetableItem, last_time: SubwayTimetableItem) -> Self {
        Self {
            first: SubwayTimeItem::new(first_time),
            last: SubwayTimeItem::new(last_time),
        }
    }
}

impl SubwayTimeItem {
    pub fn new(time_item: SubwayTimetableItem) -> Self {
        let terminal_station = SubwayStationItem::get_by_id(&time_item.terminal_station_id)
            .unwrap_or_else(
                |_| {
                    SubwayStationItem {
                        station_id: String::from(""),
                        route_id: 0,
                        station_name: String::from(""),
                        station_sequence: 0,
                        cumulative_time: 0.0,
                    }
                }
            )
            .station_name;
        Self {
            terminal_station,
            time: time_item.departure_time.format("%H:%M").to_string(),
        }
    }
}