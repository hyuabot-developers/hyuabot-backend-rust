use diesel::prelude::*;
use serde::Serialize;

use crate::db::connection;
use crate::schema::bus_stop::dsl::*;

#[derive(Queryable, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BusStopItem {
    #[diesel(sql_type = Integer)]
    pub stop_id: i32,
    #[diesel(sql_type = Text)]
    pub stop_name: Option<String>,
    #[diesel(sql_type = Integer)]
    pub district_code: i32,
    #[diesel(sql_type = Text)]
    pub mobile_number: String,
    #[diesel(sql_type = Text)]
    pub region_name: String,
    #[diesel(sql_type = Float)]
    pub latitude: f64,
    #[diesel(sql_type = Float)]
    pub longitude: f64,
}

impl BusStopItem {
    pub fn find_all() -> Result<Vec<Self>, diesel::result::Error> {
        let mut conn = connection().unwrap_or_else(|_| panic!("Failed to get DB connection"));
        let stops = bus_stop
            .load::<BusStopItem>(&mut conn)?;
        Ok(stops)
    }

    pub fn find_by_id(stop_id_query: &str) -> Result<Vec<BusStopItem>, diesel::result::Error> {
        let mut conn = connection().unwrap_or_else(|_| panic!("Failed to get DB connection"));
        let stops = bus_stop
            .filter(stop_name.like(format!("%{}%", stop_id_query)))
            .load::<BusStopItem>(&mut conn)?;
        Ok(stops)
    }

    pub fn get_one_by_id(stop_id_query: &i32) -> Result<BusStopItem, diesel::result::Error> {
        let mut conn = connection().unwrap_or_else(|_| panic!("Failed to get DB connection"));
        let stop = bus_stop
            .filter(stop_id.eq(stop_id_query))
            .first::<BusStopItem>(&mut conn)?;
        Ok(stop)
    }
}