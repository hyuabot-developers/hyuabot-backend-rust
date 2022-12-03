use diesel::prelude::*;
use serde::Serialize;

use crate::db::connection;
use crate::schema::shuttle_stop::dsl::*;


#[derive(Queryable, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ShuttleStopItem {
    #[diesel(sql_type = Text)]
    pub stop_name: String,
    #[diesel(sql_type = Float)]
    pub latitude: Option<f64>,
    #[diesel(sql_type = Float)]
    pub longitude: Option<f64>,
}

impl ShuttleStopItem {
    pub fn find_all() -> Result<Vec<ShuttleStopItem>, diesel::result::Error> {
        let mut conn = connection().unwrap_or_else(|_| panic!("Failed to get DB connection"));
        let stops = shuttle_stop
            .load::<ShuttleStopItem>(&mut conn)?;
        Ok(stops)
    }

    pub fn find_by_name(stop_name_query: &str) -> Result<Vec<ShuttleStopItem>, diesel::result::Error> {
        let mut conn = connection().unwrap_or_else(|_| panic!("Failed to get DB connection"));
        let stops = shuttle_stop
            .filter(stop_name.like(format!("%{}%", stop_name_query)))
            .load::<ShuttleStopItem>(&mut conn)?;
        Ok(stops)
    }
}