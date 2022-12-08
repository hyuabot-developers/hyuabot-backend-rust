use chrono::NaiveDateTime;
use diesel::prelude::*;

use crate::db::connection;
use crate::schema::bus_realtime::dsl::*;

#[derive(Queryable)]
pub struct BusRealtimeItem {
    #[diesel(sql_type = Integer)]
    pub stop_id: i32,
    #[diesel(sql_type = Integer)]
    pub route_id: i32,
    #[diesel(sql_type = Integer)]
    pub arrival_sequence: i32,
    #[diesel(sql_type = Integer)]
    pub remaining_stop_count: i32,
    #[diesel(sql_type = Integer)]
    pub remaining_seat_count: i32,
    #[diesel(sql_type = Integer)]
    pub remaining_time: i32,
    #[diesel(sql_type = Boolean)]
    pub low_plate: bool,
    #[diesel(sql_type = Timestamp)]
    pub last_updated_at: NaiveDateTime,
}

impl BusRealtimeItem {
    pub fn find_by_stop_and_route_id(route_id_query: &i32, stop_id_query: &i32) -> Result<Vec<Self>, diesel::result::Error> {
        let mut conn = connection().unwrap_or_else(|_| panic!("Failed to get DB connection"));
        let realtime_list = bus_realtime
            .filter(stop_id.eq(stop_id_query))
            .filter(route_id.eq(route_id_query))
            .order(arrival_sequence.asc())
            .load::<BusRealtimeItem>(&mut conn)?;
        Ok(realtime_list)
    }
}