use crate::model::subway::realtime::SubwayRealtimeItem;
use crate::model::subway::station::SubwayStationItem;
use crate::model::subway::timetable::SubwayTimetableItem;
use crate::utils::subway::get_subway_weekday;
use chrono::{Local, NaiveTime};
use serde::Serialize;

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
    pub station: String,
    pub route: i32,
    pub name: String,
    pub sequence: i32,
    pub running_time: SubwayStationItemRunningTime,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubwayStationItemRunningTime {
    pub up: SubwayStationItemFirstLastTime,
    pub down: SubwayStationItemFirstLastTime,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubwayStationItemFirstLastTime {
    pub first: SubwayTimeItem,
    pub last: SubwayTimeItem,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubwayTimeItem {
    pub destination: String,
    pub time: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubwayStationArrivalResponse {
    pub up: SubwayStationArrivalHeading,
    pub down: SubwayStationArrivalHeading,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubwayStationArrivalHeading {
    pub realtime: Vec<SubwayStationRealtimeArrivalItem>,
    pub timetable: Vec<SubwayStationTimetableArrivalItem>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubwayStationRealtimeArrivalItem {
    pub destination: String,
    pub current: String,
    pub time: i32,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubwayStationTimetableArrivalItem {
    pub destination: String,
    pub time: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubwayStationTimetableResponse {
    pub up: SubwayStationTimetableHeading,
    pub down: SubwayStationTimetableHeading,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubwayStationTimetableHeading {
    pub weekdays: Vec<SubwayStationTimetableArrivalItem>,
    pub weekends: Vec<SubwayStationTimetableArrivalItem>,
}

impl SubwayStationListResponse {
    pub fn new(station_list: Vec<SubwayStationItem>) -> Self {
        Self {
            station_list: station_list
                .into_iter()
                .map(SubwayStationListItem::new)
                .collect(),
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
        let up_first_train = SubwayTimetableItem::get_first_train_by_heading(
            &station_item.station_id,
            &get_subway_weekday(),
            "up",
        )
        .unwrap_or_else(|_| SubwayTimetableItem {
            station_id: String::from(""),
            terminal_station_name: String::from(""),
            departure_time: NaiveTime::parse_from_str("00:00:00", "%H:%M:%S").unwrap(),
            weekday: String::from(""),
            up_down_type: String::from(""),
        });
        let down_first_train = SubwayTimetableItem::get_first_train_by_heading(
            &station_item.station_id,
            &get_subway_weekday(),
            "down",
        )
        .unwrap_or_else(|_| SubwayTimetableItem {
            station_id: String::from(""),
            terminal_station_name: String::from(""),
            departure_time: NaiveTime::parse_from_str("00:00:00", "%H:%M:%S").unwrap(),
            weekday: String::from(""),
            up_down_type: String::from(""),
        });
        let up_last_train = SubwayTimetableItem::get_last_train_by_heading(
            &station_item.station_id,
            &get_subway_weekday(),
            "up",
        )
        .unwrap_or_else(|_| SubwayTimetableItem {
            station_id: String::from(""),
            terminal_station_name: String::from(""),
            departure_time: NaiveTime::parse_from_str("00:00:00", "%H:%M:%S").unwrap(),
            weekday: String::from(""),
            up_down_type: String::from(""),
        });
        let down_last_train = SubwayTimetableItem::get_last_train_by_heading(
            &station_item.station_id,
            &get_subway_weekday(),
            "down",
        )
        .unwrap_or_else(|_| SubwayTimetableItem {
            station_id: String::from(""),
            terminal_station_name: String::from(""),
            departure_time: NaiveTime::parse_from_str("00:00:00", "%H:%M:%S").unwrap(),
            weekday: String::from(""),
            up_down_type: String::from(""),
        });
        Self {
            station: station_item.station_id.clone(),
            route: station_item.route_id,
            name: station_item.station_name,
            sequence: station_item.station_sequence,
            running_time: SubwayStationItemRunningTime::new(
                up_first_train,
                up_last_train,
                down_first_train,
                down_last_train,
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
        Self {
            destination: time_item.terminal_station_name,
            time: time_item.departure_time.format("%H:%M").to_string(),
        }
    }
}

impl SubwayStationArrivalResponse {
    pub fn new(station_id: &str) -> Self {
        Self {
            up: SubwayStationArrivalHeading::new(
                &SubwayRealtimeItem::find_by_station(station_id, "up").unwrap(),
                &SubwayTimetableItem::get_train_by_heading(station_id, &get_subway_weekday(), "up")
                    .unwrap(),
            ),
            down: SubwayStationArrivalHeading::new(
                &SubwayRealtimeItem::find_by_station(station_id, "down").unwrap(),
                &SubwayTimetableItem::get_train_by_heading(
                    station_id,
                    &get_subway_weekday(),
                    "down",
                )
                .unwrap(),
            ),
        }
    }
}

impl SubwayStationArrivalHeading {
    pub fn new(
        realtime_arrival_list: &[SubwayRealtimeItem],
        timetable_list: &[SubwayTimetableItem],
    ) -> Self {
        let now = Local::now();
        let last_realtime_item = realtime_arrival_list.last().unwrap();
        Self {
            realtime: realtime_arrival_list
                .iter()
                .map(SubwayStationRealtimeArrivalItem::new)
                .collect(),
            timetable: timetable_list
                .iter()
                .filter(|timetable_item| {
                    (timetable_item.departure_time - now.time()).num_minutes()
                        > last_realtime_item.remaining_time as i64
                            - (now.naive_local() - last_realtime_item.last_updated_time)
                                .num_minutes()
                            + 2
                })
                .map(SubwayStationTimetableArrivalItem::new)
                .collect(),
        }
    }
}

impl SubwayStationRealtimeArrivalItem {
    pub fn new(realtime_item: &SubwayRealtimeItem) -> Self {
        Self {
            destination: realtime_item.terminal_station_name.clone(),
            current: realtime_item.current_station_name.clone(),
            time: realtime_item.remaining_time,
        }
    }
}

impl SubwayStationTimetableArrivalItem {
    pub fn new(timetable_item: &SubwayTimetableItem) -> Self {
        Self {
            destination: timetable_item.terminal_station_name.clone(),
            time: timetable_item.departure_time.format("%H:%M").to_string(),
        }
    }
}

impl SubwayStationTimetableResponse {
    pub fn new(timetable: &Vec<SubwayTimetableItem>) -> Self {
        Self {
            up: SubwayStationTimetableHeading::new(timetable, "up"),
            down: SubwayStationTimetableHeading::new(timetable, "down"),
        }
    }
}

impl SubwayStationTimetableHeading {
    pub fn new(timetable: &Vec<SubwayTimetableItem>, heading: &str) -> Self {
        Self {
            weekdays: timetable
                .into_iter()
                .filter(|item| item.weekday == "weekdays" && item.up_down_type == heading)
                .map(|timetable_item| SubwayStationTimetableArrivalItem::new(&timetable_item))
                .collect(),
            weekends: timetable
                .into_iter()
                .filter(|item| item.weekday == "weekends" && item.up_down_type == heading)
                .map(|timetable_item| SubwayStationTimetableArrivalItem::new(&timetable_item))
                .collect(),
        }
    }
}
