use diesel::prelude::*;
use serde::Serialize;

use crate::db::connection;
use crate::schema::shuttle_route::dsl::*;

#[derive(Queryable, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ShuttleRouteItem {
    #[diesel(sql_type = Text)]
    pub route_name: String,
    #[diesel(sql_type = Text)]
    pub description_korean: Option<String>,
    #[diesel(sql_type = Text)]
    pub description_english: Option<String>,
}

impl ShuttleRouteItem {
    pub fn find_all() -> Result<Vec<Self>, diesel::result::Error> {
        let mut conn = connection().unwrap_or_else(|_| panic!("Failed to get DB connection"));
        let routes = shuttle_route.load::<ShuttleRouteItem>(&mut conn)?;
        Ok(routes)
    }

    pub fn find_by_name(route_name_query: &str) -> Result<Vec<Self>, diesel::result::Error> {
        let mut conn = connection().unwrap_or_else(|_| panic!("Failed to get DB connection"));
        let routes = shuttle_route
            .filter(route_name.like(format!("%{}%", route_name_query)))
            .load::<ShuttleRouteItem>(&mut conn)?;
        Ok(routes)
    }

    pub fn get_one_by_name(route_name_query: &str) -> Result<Self, diesel::result::Error> {
        let mut conn = connection().unwrap_or_else(|_| panic!("Failed to get DB connection"));
        let route = shuttle_route
            .filter(route_name.eq(route_name_query))
            .first::<ShuttleRouteItem>(&mut conn)?;
        Ok(route)
    }
}
