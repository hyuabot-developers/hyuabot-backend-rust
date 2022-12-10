use chrono::NaiveTime;
use diesel::prelude::*;

use crate::db::connection;
use crate::schema::subway_route_station::dsl::*;
use crate::schema::subway_timetable::dsl::*;
use crate::schema::subway_timetable::dsl as subway_timetable_table;

#[derive(Queryable)]
pub struct SubwayTimetableItem {
    #[diesel(sql_type = Text)]
    pub station_id: String,
    #[diesel(sql_type = Text)]
    pub terminal_station_name: String,
    #[diesel(sql_type = Time)]
    pub departure_time: NaiveTime,
    #[diesel(sql_type = Text)]
    pub weekday: String,
    #[diesel(sql_type = Text)]
    pub up_down_type: String,
}

impl SubwayTimetableItem {
    pub fn get_first_train_by_heading(station_id_query: &str, weekday_query: &str, up_down_type_query: &str) -> Result<Self, diesel::result::Error> {
        let mut conn = connection().unwrap_or_else(|_| panic!("Failed to get DB connection"));
        Ok(
            subway_timetable
                .inner_join(subway_route_station)
                .select((
                    subway_timetable_table::station_id, station_name,
                    departure_time, weekday, up_down_type
                ))
                .filter(subway_timetable_table::station_id.eq(station_id_query))
                .filter(weekday.eq(weekday_query))
                .filter(up_down_type.eq(up_down_type_query))
                .filter(departure_time.gt(NaiveTime::parse_from_str("04:00:00", "%H:%M:%S").unwrap()))
                .order(departure_time.asc())
                .limit(1)
                .first::<SubwayTimetableItem>(&mut conn)?
        )
    }

    pub fn get_last_train_by_heading(station_id_query: &str, weekday_query: &str, up_down_type_query: &str) -> Result<Self, diesel::result::Error> {
        let mut conn = connection().unwrap_or_else(|_| panic!("Failed to get DB connection"));
        Ok(subway_timetable
            .inner_join(subway_route_station)
            .select((
                subway_timetable_table::station_id, station_name,
                departure_time, weekday, up_down_type
            ))
            .filter(subway_timetable_table::station_id.eq(station_id_query))
            .filter(weekday.eq(weekday_query))
            .filter(up_down_type.eq(up_down_type_query))
            .filter(departure_time.lt(NaiveTime::parse_from_str("04:00:00", "%H:%M:%S").unwrap()))
            .order(departure_time.desc())
            .limit(1)
            .first::<SubwayTimetableItem>(&mut conn).unwrap_or_else(
                |_| {
                    subway_timetable
                        .inner_join(subway_route_station)
                        .select((
                            subway_timetable_table::station_id, station_name,
                            departure_time, weekday, up_down_type
                        ))
                        .filter(subway_timetable_table::station_id.eq(station_id_query))
                        .filter(weekday.eq(weekday_query))
                        .filter(up_down_type.eq(up_down_type_query))
                        .order(departure_time.desc())
                        .limit(1)
                        .first::<SubwayTimetableItem>(&mut conn).unwrap_or_else(
                            |_| {
                                SubwayTimetableItem {
                                    station_id: String::from(""),
                                    terminal_station_name: String::from(""),
                                    departure_time: NaiveTime::parse_from_str("00:00:00", "%H:%M:%S").unwrap(),
                                    weekday: String::from(""),
                                    up_down_type: String::from(""),
                                }
                            }
                        )
                }
            )
        )
    }

    pub fn get_train_by_heading(station_id_query: &str, weekday_query: &str, up_down_type_query: &str)
        -> Result<Vec<Self>, diesel::result::Error> {
        let mut conn = connection().unwrap_or_else(|_| panic!("Failed to get DB connection"));
        Ok(
            subway_timetable
                .inner_join(subway_route_station)
                .select((
                    subway_timetable_table::station_id, station_name,
                    departure_time, weekday, up_down_type
                ))
                .filter(subway_timetable_table::station_id.eq(station_id_query))
                .filter(weekday.eq(weekday_query))
                .filter(up_down_type.eq(up_down_type_query))
                .order(departure_time.asc())
                .load::<SubwayTimetableItem>(&mut conn)?
        )
    }
}