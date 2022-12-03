use diesel::prelude::*;
use serde::Serialize;

use crate::db::connection;
use crate::schema::shuttle_route_stop::dsl::*;


#[derive(Queryable, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ShuttleRouteStopItem {
    #[diesel(sql_type = Text)]
    pub route_name: String,
    #[diesel(sql_type = Text)]
    pub stop_name: String,
    #[diesel(sql_type = Integer)]
    pub stop_order: Option<i32>,
    #[diesel(sql_type = Integer)]
    pub cumulative_time: Option<i32>,
}


impl ShuttleRouteStopItem {
    pub fn get_stop_list_by_route_name(route_name_query: &str) -> Result<Vec<ShuttleRouteStopItem>, diesel::result::Error> {
        let mut conn = connection().unwrap_or_else(|_| panic!("Failed to get DB connection"));
        let stops = shuttle_route_stop
            .filter(route_name.eq(route_name_query))
            .order(stop_order.asc())
            .load::<ShuttleRouteStopItem>(&mut conn)?;
        Ok(stops)
    }
}
