use chrono::NaiveTime;
use diesel::prelude::*;

use crate::db::connection;
use crate::schema::bus_timetable::dsl::*;
use crate::utils::bus::get_bus_weekday;

#[derive(Queryable)]
pub struct BusTimetableItem {
    #[diesel(sql_type = Integer)]
    pub route_id: i32,
    #[diesel(sql_type = Integer)]
    pub start_stop_id: i32,
    #[diesel(sql_type = Time)]
    pub departure_time: NaiveTime,
    #[diesel(sql_type = Text)]
    pub weekday: String,
}

impl BusTimetableItem {
    pub fn find_by_stop_and_route_id(route_id_query: &i32, start_stop_id_query: &i32) -> Result<Vec<Self>, diesel::result::Error> {
        let mut conn = connection().unwrap_or_else(|_| panic!("Failed to get DB connection"));
        let timetable = bus_timetable
            .filter(start_stop_id.eq(start_stop_id_query))
            .filter(route_id.eq(route_id_query))
            .order(departure_time.asc())
            .load::<Self>(&mut conn)?;
        Ok(timetable)
    }

    pub fn find_by_weekday_stop_and_route_id(
        route_id_query: &i32, start_stop_id_query: &i32,
        weekday_query: Option<&str>, show_all: Option<&bool>) -> Result<Vec<Self>, diesel::result::Error> {
        let mut conn = connection().unwrap_or_else(|_| panic!("Failed to get DB connection"));
        let now = chrono::Local::now().time();
        if show_all.eq(&Some(&true)) {
            Ok(match weekday_query {
                Some(weekday_value) => bus_timetable
                    .filter(start_stop_id.eq(start_stop_id_query))
                    .filter(route_id.eq(route_id_query))
                    .filter(weekday.eq(weekday_value))
                    .order(departure_time.asc())
                    .load::<Self>(&mut conn)?,
                None => bus_timetable
                    .filter(start_stop_id.eq(start_stop_id_query))
                    .filter(route_id.eq(route_id_query))
                    .filter(weekday.eq(get_bus_weekday()))
                    .order(departure_time.asc())
                    .load::<Self>(&mut conn)?,
            })
        } else {
            Ok(match weekday_query {
                Some(weekday_value) => bus_timetable
                    .filter(start_stop_id.eq(start_stop_id_query))
                    .filter(route_id.eq(route_id_query))
                    .filter(weekday.eq(weekday_value))
                    .filter(departure_time.gt(now))
                    .order(departure_time.asc())
                    .load::<Self>(&mut conn)?,
                None => bus_timetable
                    .filter(start_stop_id.eq(start_stop_id_query))
                    .filter(route_id.eq(route_id_query))
                    .filter(weekday.eq(get_bus_weekday()))
                    .filter(departure_time.gt(now))
                    .order(departure_time.asc())
                    .load::<Self>(&mut conn)?,
            })
        }
    }
}