use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::db::connection;
use crate::schema::shuttle_route::dsl::*;

#[derive(Serialize, Deserialize)]
pub struct ShuttleRouteItem {
    pub route_name: String,
    pub description_korean: Option<String>,
    pub description_english: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ShuttleRouteList {
    pub routes: Vec<ShuttleRouteItem>,
}

impl ShuttleRouteList {
    pub fn find_all() -> Result<Self, diesel::result::Error> {
        let conn = connection().unwrap_or_else(|_| panic!("Failed to get connection from pool"));
        let routes = shuttle_route.load::<ShuttleRouteItem>(conn)?;
        Ok(Self { routes })
    }
}