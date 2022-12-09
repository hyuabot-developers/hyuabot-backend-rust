use diesel::prelude::*;

use serde::Serialize;
use crate::db::connection;
use crate::schema::subway_route_station::dsl::*;

#[derive(Queryable, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubwayStationItem {
    #[diesel(sql_type = Text)]
    pub station_id: String,
    #[diesel(sql_type = Integer)]
    pub route_id: i32,
    #[diesel(sql_type = Text)]
    pub station_name: String,
    #[diesel(sql_type = Integer)]
    pub station_sequence: i32,
    #[diesel(sql_type = Double)]
    pub cumulative_time: f64,
}

impl SubwayStationItem {
    pub fn find_all() -> Result<Vec<SubwayStationItem>, diesel::result::Error> {
        let mut conn = connection().unwrap_or_else(|_| panic!("Failed to get DB connection"));
        let station_list = subway_route_station
            .load::<SubwayStationItem>(&mut conn)?;
        Ok(station_list)
    }

    pub fn find_by_name(station_name_query: &str) -> Result<Vec<SubwayStationItem>, diesel::result::Error> {
        let mut conn = connection().unwrap_or_else(|_| panic!("Failed to get DB connection"));
        let station_list = subway_route_station
            .filter(station_name.like(format!("%{}%", station_name_query)))
            .load::<SubwayStationItem>(&mut conn)?;
        Ok(station_list)
    }

    pub fn get_by_id(station_id_query: &str) -> Result<SubwayStationItem, diesel::result::Error> {
        let mut conn = connection().unwrap_or_else(|_| panic!("Failed to get DB connection"));
        let station = subway_route_station
            .filter(station_id.eq(station_id_query))
            .first::<SubwayStationItem>(&mut conn)?;
        Ok(station)
    }
}