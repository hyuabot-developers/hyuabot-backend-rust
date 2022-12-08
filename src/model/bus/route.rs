use chrono::NaiveTime;
use diesel::prelude::*;
use serde::Serialize;

use crate::db::connection;
use crate::schema::bus_route::dsl::*;

#[derive(Queryable)]
pub struct BusRouteItem {
    #[diesel(sql_type = Integer)]
    pub company_id: Option<i32>,
    #[diesel(sql_type = Text)]
    pub company_name: String,
    #[diesel(sql_type = Text)]
    pub company_telephone: String,
    #[diesel(sql_type = Integer)]
    pub district_code: i32,
    #[diesel(sql_type = Time)]
    pub up_first_time: NaiveTime,
    #[diesel(sql_type = Time)]
    pub up_last_time: NaiveTime,
    #[diesel(sql_type = Time)]
    pub down_first_time: NaiveTime,
    #[diesel(sql_type = Time)]
    pub down_last_time: NaiveTime,
    #[diesel(sql_type = Integer)]
    pub start_stop_id: i32,
    #[diesel(sql_type = Integer)]
    pub end_stop_id: i32,
    #[diesel(sql_type = Integer)]
    pub route_id: i32,
    #[diesel(sql_type = Text)]
    pub route_name: String,
    #[diesel(sql_type = Text)]
    pub route_type_code: String,
    #[diesel(sql_type = Text)]
    pub route_type_name: String,
}

impl BusRouteItem {
    pub fn find_all() -> Result<Vec<Self>, diesel::result::Error> {
        let mut conn = connection().unwrap_or_else(|_| panic!("Failed to get DB connection"));
        let routes = bus_route
            .order(route_id.asc())
            .load::<Self>(&mut conn)?;
        Ok(routes)
    }

    pub fn find_by_name(route_name_query: &str) -> Result<Vec<Self>, diesel::result::Error> {
        let mut conn = connection().unwrap_or_else(|_| panic!("Failed to get DB connection"));
        let route = bus_route
            .filter(route_name.like(format!("%{}%", route_name_query)))
            .load::<Self>(&mut conn)?;
        Ok(route)
    }
}