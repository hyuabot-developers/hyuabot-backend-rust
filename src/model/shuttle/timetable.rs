use chrono::NaiveTime;
use diesel::prelude::*;

use crate::db::connection;
use crate::schema::shuttle_timetable::dsl::*;


#[derive(Queryable)]
pub struct ShuttleTimeTableItem {
    #[diesel(sql_type = Text)]
    pub period_type: String,
    #[diesel(sql_type = diesel::sql_types::Bool)]
    pub weekday: bool,
    #[diesel(sql_type = Text)]
    pub route_name: String,
    #[diesel(sql_type = Time)]
    pub departure_time: NaiveTime,
    #[diesel(sql_type = Text)]
    pub start_stop: String,
}

impl ShuttleTimeTableItem {
    pub fn get_timetable_by_route_name(route_name_query: &str) -> Result<Vec<ShuttleTimeTableItem>, diesel::result::Error> {
        let mut conn = connection().unwrap_or_else(|_| panic!("Failed to get DB connection"));
        let timetable = shuttle_timetable
            .filter(route_name.eq(route_name_query))
            .order(departure_time.asc())
            .load::<ShuttleTimeTableItem>(&mut conn)?;
        Ok(timetable)
    }
}