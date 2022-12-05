use diesel::prelude::*;
use serde::Serialize;

use crate::db::connection;
use crate::schema::shuttle_route::dsl::*;
use crate::schema::shuttle_route::dsl as shuttle_route_table;
use crate::schema::shuttle_route_stop::dsl::*;
use crate::schema::shuttle_route_stop::dsl as shuttle_route_stop_table;


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

#[derive(Queryable, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ShuttleRouteStopItemWithDescription {
    #[diesel(sql_type = Text)]
    pub route_name: String,
    #[diesel(sql_type = Integer)]
    pub cumulative_time: Option<i32>,
    #[diesel(sql_type = Text)]
    pub description_korean: Option<String>,
    #[diesel(sql_type = Text)]
    pub description_english: Option<String>,
}


impl ShuttleRouteStopItem {
    pub fn get_stop_list_by_route_name(route_name_query: &str) -> Result<Vec<ShuttleRouteStopItem>, diesel::result::Error> {
        let mut conn = connection().unwrap_or_else(|_| panic!("Failed to get DB connection"));
        let stops = shuttle_route_stop
            .filter(shuttle_route_stop_table::route_name.eq(route_name_query))
            .order(stop_order.asc())
            .load::<ShuttleRouteStopItem>(&mut conn)?;
        Ok(stops)
    }

    pub fn get_route_list_by_stop_name(stop_name_query: &str) -> Result<Vec<ShuttleRouteStopItemWithDescription>, diesel::result::Error> {
        let mut conn = connection().unwrap_or_else(|_| panic!("Failed to get DB connection"));
        let routes = shuttle_route_stop.inner_join(shuttle_route)
            .select((shuttle_route_table::route_name, cumulative_time, route_description_korean, route_description_english))
            .filter(stop_name.eq(stop_name_query))
            .order(shuttle_route_table::route_name.asc())
            .load::<ShuttleRouteStopItemWithDescription>(&mut conn)?;
        Ok(routes)
    }

    pub fn get_route_item_by_stop_name(stop_name_query: &str, route_name_query: &str) -> Result<ShuttleRouteStopItemWithDescription, diesel::result::Error> {
        let mut conn = connection().unwrap_or_else(|_| panic!("Failed to get DB connection"));
        let routes = shuttle_route_stop.inner_join(shuttle_route)
            .select((shuttle_route_table::route_name, cumulative_time, route_description_korean, route_description_english))
            .filter(stop_name.eq(stop_name_query))
            .filter(shuttle_route_table::route_name.eq(route_name_query))
            .first::<ShuttleRouteStopItemWithDescription>(&mut conn)?;
        Ok(routes)
    }
}
