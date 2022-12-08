use diesel::prelude::*;
use serde::Serialize;

use crate::db::connection;
use crate::schema::bus_route::dsl::*;
use crate::schema::bus_route::dsl as bus_route_table;
use crate::schema::bus_route_stop::dsl::*;
use crate::schema::bus_route_stop::dsl as bus_route_stop_table;


#[derive(Queryable, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BusRouteStopItem {
    #[diesel(sql_type = Integer)]
    pub route_id: i32,
    #[diesel(sql_type = Text)]
    pub route_name: String,
    #[diesel(sql_type = Integer)]
    pub stop_id: i32,
    #[diesel(sql_type = Integer)]
    pub stop_sequence: i32,
    #[diesel(sql_type = Integer)]
    pub start_stop_id: i32,
}

impl BusRouteStopItem {
    pub fn find_by_stop_id(stop_id_query: &i32) -> Result<Vec<Self>, diesel::result::Error> {
        let mut conn = connection().unwrap_or_else(|_| panic!("Failed to get DB connection"));
        let stops = bus_route_stop
            .inner_join(bus_route)
            .select((bus_route_table::route_id, route_name, stop_id, stop_sequence, bus_route_stop_table::start_stop_id))
            .filter(stop_id.eq(stop_id_query))
            .load::<BusRouteStopItem>(&mut conn)?;
        Ok(stops)
    }
}