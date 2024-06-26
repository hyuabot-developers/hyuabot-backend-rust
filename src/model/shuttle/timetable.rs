use chrono::{Duration, NaiveTime};
use diesel::prelude::*;
use std::ops::Sub;

use crate::db::connection;
use crate::model::shuttle::route_stop::ShuttleRouteStopItemWithDescription;
use crate::schema::shuttle_period::dsl as shuttle_period_table;
use crate::schema::shuttle_period::dsl::*;
use crate::schema::shuttle_route::dsl as shuttle_route_table;
use crate::schema::shuttle_route::dsl::*;
use crate::schema::shuttle_route_stop::dsl as shuttle_route_stop_table;
use crate::schema::shuttle_route_stop::dsl::*;
use crate::schema::shuttle_stop::dsl as shuttle_stop_table;
use crate::schema::shuttle_stop::dsl::*;
use crate::schema::shuttle_timetable::dsl as shuttle_timetable_table;
use crate::schema::shuttle_timetable::dsl::*;

#[derive(Queryable)]
pub struct ShuttleTimeTableItem {
    #[diesel(sql_type = Text)]
    pub period_type: String,
    #[diesel(sql_type = diesel::sql_types::Bool)]
    pub weekday: bool,
    #[diesel(sql_type = Text)]
    pub route_name: String,
    #[diesel(sql_type = Time)]
    pub departure_time: NaiveTime,
    #[diesel(sql_type = Text)]
    pub start_stop: String,
}

#[derive(Queryable)]
pub struct EntireShuttleTimeTableItem {
    #[diesel(sql_type = Text)]
    pub route_name: String,
    #[diesel(sql_type = Text)]
    pub stop_name: String,
    #[diesel(sql_type = Bool)]
    pub weekday: bool,
    #[diesel(sql_type = Time)]
    pub departure_time: NaiveTime,
    #[diesel(sql_type = Integer)]
    pub cumulative_time: Option<i32>,
}

#[derive(Queryable)]
pub struct ShuttleTimeTableByShuttleStopItem {
    #[diesel(sql_type = Text)]
    pub route_name: String,
    #[diesel(sql_type = Time)]
    pub departure_time: NaiveTime,
}

#[derive(Queryable)]
pub struct ShuttleStopTimeTableItem {
    #[diesel(sql_type = Double)]
    pub latitude: Option<f64>,
    #[diesel(sql_type = Double)]
    pub longitude: Option<f64>,
    #[diesel(sql_type = Text)]
    pub route_name: String,
    #[diesel(sql_type = Text)]
    pub route_description_korean: Option<String>,
    #[diesel(sql_type = Text)]
    pub route_description_english: Option<String>,
    #[diesel(sql_type = Integer)]
    pub cumulative_time: Option<i32>,
    #[diesel(sql_type = Bool)]
    pub weekday: bool,
    #[diesel(sql_type = Time)]
    pub departure_time: NaiveTime,
}

impl ShuttleTimeTableItem {
    pub fn get_timetable_by_route_name(
        route_name_query: &str,
        period_query: &str,
    ) -> Result<Vec<ShuttleTimeTableItem>, diesel::result::Error> {
        let mut conn = connection().unwrap_or_else(|_| panic!("Failed to get DB connection"));
        let timetable = shuttle_timetable
            .filter(shuttle_timetable_table::route_name.eq(route_name_query))
            .filter(shuttle_timetable_table::period_type.eq(period_query))
            .order(departure_time.asc())
            .load::<ShuttleTimeTableItem>(&mut conn)?;
        Ok(timetable)
    }
}

impl ShuttleTimeTableByShuttleStopItem {
    pub fn get_timetable_by_route_stop_name(
        period_query: &str,
        weekday_query: &bool,
        route_item: &ShuttleRouteStopItemWithDescription,
        show_all: &Option<bool>,
    ) -> Result<Vec<ShuttleTimeTableByShuttleStopItem>, diesel::result::Error> {
        let mut conn = connection().unwrap_or_else(|_| panic!("Failed to get DB connection"));
        let mut timetable = Vec::new();
        let now = chrono::Local::now().naive_local();
        if show_all.is_some() && show_all.unwrap() {
            let mut route_timetable = shuttle_timetable
                .select((shuttle_timetable_table::route_name, departure_time))
                .filter(shuttle_timetable_table::route_name.eq(&route_item.route_name))
                .filter(shuttle_timetable_table::period_type.eq(period_query))
                .filter(weekday.eq(weekday_query))
                .order(departure_time.asc())
                .load::<ShuttleTimeTableByShuttleStopItem>(&mut conn)?;
            timetable.append(&mut route_timetable);
        } else {
            let mut timetable_by_route = shuttle_timetable
                .select((shuttle_timetable_table::route_name, departure_time))
                .filter(shuttle_timetable_table::route_name.eq(&route_item.route_name))
                .filter(shuttle_timetable_table::period_type.eq(period_query))
                .filter(weekday.eq(weekday_query))
                .filter(departure_time.gt(now.time().sub(Duration::minutes(
                    route_item.cumulative_time.unwrap_or(0) as i64,
                ))))
                .order(departure_time.asc())
                .load::<ShuttleTimeTableByShuttleStopItem>(&mut conn)?;
            timetable.append(&mut timetable_by_route);
        }
        Ok(timetable)
    }
}

impl EntireShuttleTimeTableItem {
    pub fn find_all() -> Result<Vec<Self>, diesel::result::Error> {
        let mut conn = connection().unwrap_or_else(|_| panic!("Failed to get DB connection"));
        let now = chrono::Local::now().naive_local();
        let timetable = shuttle_timetable
            .inner_join(
                shuttle_period
                    .on(shuttle_timetable_table::period_type.eq(shuttle_period_table::period_type)),
            )
            .inner_join(
                shuttle_route_stop
                    .on(shuttle_timetable_table::route_name
                        .eq(shuttle_route_stop_table::route_name)),
            )
            .select((
                shuttle_timetable_table::route_name,
                shuttle_route_stop_table::stop_name,
                weekday,
                departure_time,
                cumulative_time,
            ))
            .filter(period_start.le(now))
            .filter(period_end.gt(now))
            .load::<EntireShuttleTimeTableItem>(&mut conn)?;
        Ok(timetable)
    }
}

impl ShuttleStopTimeTableItem {
    pub fn get_timetable_by_stop_name(
        stop_query: &str,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let mut conn = connection().unwrap_or_else(|_| panic!("Failed to get DB connection"));
        let now = chrono::Local::now().naive_local();
        let timetable = shuttle_stop
            .inner_join(
                shuttle_route_stop
                    .on(shuttle_stop_table::stop_name.eq(shuttle_route_stop_table::stop_name)),
            )
            .inner_join(
                shuttle_route
                    .on(shuttle_route_stop_table::route_name.eq(shuttle_route_table::route_name)),
            )
            .inner_join(
                shuttle_timetable
                    .on(shuttle_route_stop_table::route_name
                        .eq(shuttle_timetable_table::route_name)),
            )
            .inner_join(
                shuttle_period
                    .on(shuttle_timetable_table::period_type.eq(shuttle_period_table::period_type)),
            )
            .select((
                latitude,
                longitude,
                shuttle_route_stop_table::route_name,
                route_description_korean,
                route_description_english,
                cumulative_time,
                weekday,
                departure_time,
            ))
            .filter(period_start.le(now))
            .filter(period_end.gt(now))
            .filter(shuttle_stop_table::stop_name.eq(stop_query))
            .load::<ShuttleStopTimeTableItem>(&mut conn)?;
        Ok(timetable)
    }

    pub fn get_timetable_by_stop_name_and_route_name(
        stop_query: &str,
        route_query: &str,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let mut conn = connection().unwrap_or_else(|_| panic!("Failed to get DB connection"));
        let now = chrono::Local::now().naive_local();
        let timetable = shuttle_stop
            .inner_join(
                shuttle_route_stop
                    .on(shuttle_stop_table::stop_name.eq(shuttle_route_stop_table::stop_name)),
            )
            .inner_join(
                shuttle_route
                    .on(shuttle_route_stop_table::route_name.eq(shuttle_route_table::route_name)),
            )
            .inner_join(
                shuttle_timetable
                    .on(shuttle_route_stop_table::route_name
                        .eq(shuttle_timetable_table::route_name)),
            )
            .inner_join(
                shuttle_period
                    .on(shuttle_timetable_table::period_type.eq(shuttle_period_table::period_type)),
            )
            .select((
                latitude,
                longitude,
                shuttle_route_stop_table::route_name,
                route_description_korean,
                route_description_english,
                cumulative_time,
                weekday,
                departure_time,
            ))
            .filter(shuttle_timetable_table::route_name.eq(route_query))
            .filter(period_start.le(now))
            .filter(period_end.gt(now))
            .filter(shuttle_stop_table::stop_name.eq(stop_query))
            .load::<ShuttleStopTimeTableItem>(&mut conn)?;
        Ok(timetable)
    }
}
