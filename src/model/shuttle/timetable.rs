use chrono::{Duration, NaiveTime};
use diesel::prelude::*;
use std::ops::Sub;

use crate::db::connection;
use crate::model::shuttle::route_stop::ShuttleRouteStopItemWithDescription;
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
pub struct ShuttleTimeTableByShuttleStopItem {
    #[diesel(sql_type = Text)]
    pub route_name: String,
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
            .filter(route_name.eq(route_name_query))
            .filter(period_type.eq(period_query))
            .order(departure_time.asc())
            .load::<ShuttleTimeTableItem>(&mut conn)?;
        Ok(timetable)
    }
}

impl ShuttleTimeTableByShuttleStopItem {
    pub fn get_timetable_by_stop_name(
        period_query: &str,
        weekday_query: &bool,
        route_list: &Vec<ShuttleRouteStopItemWithDescription>,
        limit: &i64,
        show_all: &Option<bool>,
    ) -> Result<Vec<ShuttleTimeTableByShuttleStopItem>, diesel::result::Error> {
        let mut conn = connection().unwrap_or_else(|_| panic!("Failed to get DB connection"));
        let mut timetable = Vec::new();
        let now = chrono::Local::now().naive_local();
        if show_all.is_some() && show_all.unwrap() {
            for route in route_list {
                let mut route_timetable = shuttle_timetable
                    .select((route_name, departure_time))
                    .filter(route_name.eq(&route.route_name))
                    .filter(period_type.eq(period_query))
                    .filter(weekday.eq(weekday_query))
                    .order(departure_time.asc())
                    .load::<ShuttleTimeTableByShuttleStopItem>(&mut conn)?;
                timetable.append(&mut route_timetable);
            }
        } else {
            for route in route_list {
                let mut timetable_by_route = shuttle_timetable
                    .select((route_name, departure_time))
                    .filter(route_name.eq(&route.route_name))
                    .filter(period_type.eq(period_query))
                    .filter(weekday.eq(weekday_query))
                    .filter(
                        departure_time.gt(now
                            .time()
                            .sub(Duration::minutes(route.cumulative_time.unwrap_or(0) as i64))),
                    )
                    .order(departure_time.asc())
                    .limit(*limit)
                    .load::<ShuttleTimeTableByShuttleStopItem>(&mut conn)?;
                timetable.append(&mut timetable_by_route);
            }
        }
        Ok(timetable)
    }

    pub fn get_timetable_by_route_stop_name(
        period_query: &str,
        weekday_query: &bool,
        route_item: &ShuttleRouteStopItemWithDescription,
        limit: &i64,
        show_all: &Option<bool>,
    ) -> Result<Vec<ShuttleTimeTableByShuttleStopItem>, diesel::result::Error> {
        let mut conn = connection().unwrap_or_else(|_| panic!("Failed to get DB connection"));
        let mut timetable = Vec::new();
        let now = chrono::Local::now().naive_local();
        if show_all.is_some() && show_all.unwrap() {
            let mut route_timetable = shuttle_timetable
                .select((route_name, departure_time))
                .filter(route_name.eq(&route_item.route_name))
                .filter(period_type.eq(period_query))
                .filter(weekday.eq(weekday_query))
                .order(departure_time.asc())
                .load::<ShuttleTimeTableByShuttleStopItem>(&mut conn)?;
            timetable.append(&mut route_timetable);
        } else {
            let mut timetable_by_route = shuttle_timetable
                .select((route_name, departure_time))
                .filter(route_name.eq(&route_item.route_name))
                .filter(period_type.eq(period_query))
                .filter(weekday.eq(weekday_query))
                .filter(departure_time.gt(now.time().sub(Duration::minutes(
                    route_item.cumulative_time.unwrap_or(0) as i64,
                ))))
                .order(departure_time.asc())
                .limit(*limit)
                .load::<ShuttleTimeTableByShuttleStopItem>(&mut conn)?;
            timetable.append(&mut timetable_by_route);
        }
        Ok(timetable)
    }
}
