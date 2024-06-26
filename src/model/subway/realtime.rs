use chrono::NaiveDateTime;
use diesel::prelude::*;

use crate::db::connection;
use crate::schema::subway_realtime::dsl as subway_realtime_table;
use crate::schema::subway_realtime::dsl::*;
use crate::schema::subway_route_station::dsl::*;

#[derive(Queryable)]
pub struct SubwayRealtimeItem {
    #[diesel(sql_type = Text)]
    pub station_id: String,
    #[diesel(sql_type = Integer)]
    pub arrival_sequence: i32,
    #[diesel(sql_type = Text)]
    pub current_station_name: String,
    #[diesel(sql_type = Integer)]
    pub remaining_stop_count: i32,
    #[diesel(sql_type = Integer)]
    pub remaining_time: i32,
    #[diesel(sql_type = Text)]
    pub up_down_type: String,
    #[diesel(sql_type = Text)]
    pub terminal_station_name: String,
    #[diesel(sql_type = Text)]
    pub train_number: String,
    #[diesel(sql_type = Timestamp)]
    pub last_updated_time: NaiveDateTime,
    #[diesel(sql_type = Bool)]
    pub is_express_train: bool,
    #[diesel(sql_type = Bool)]
    pub is_last_train: bool,
    #[diesel(sql_type = Integer)]
    pub status_code: i32,
}

impl SubwayRealtimeItem {
    pub fn find_by_station(
        station_id_query: &str,
        heading: &str,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        let mut conn = connection().unwrap_or_else(|_| panic!("Failed to get DB connection"));
        let route = subway_realtime
            .inner_join(subway_route_station)
            .select((
                subway_realtime_table::station_id,
                arrival_sequence,
                current_station_name,
                remaining_stop_count,
                remaining_time,
                up_down_type,
                station_name,
                train_number,
                last_updated_time,
                is_express_train,
                is_last_train,
                status_code,
            ))
            .filter(subway_realtime_table::station_id.eq(station_id_query))
            .filter(up_down_type.eq((heading == "up").to_string()))
            .order(remaining_time.asc())
            .load::<Self>(&mut conn)?;
        Ok(route)
    }
}
