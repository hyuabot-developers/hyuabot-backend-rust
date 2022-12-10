use chrono::NaiveDate;
use diesel::prelude::*;

use crate::db::connection;
use crate::schema::menu::dsl::*;


#[derive(Queryable)]
pub struct MenuItem {
    #[diesel(sql_type = Integer)]
    pub restaurant_id: i32,
    #[diesel(sql_type = Date)]
    pub feed_date: NaiveDate,
    #[diesel(sql_type = Text)]
    pub time_type: String,
    #[diesel(sql_type = Text)]
    pub menu_food: String,
    #[diesel(sql_type = Text)]
    pub menu_price: String,
}

impl MenuItem {
    pub fn find_by_restaurant_id(restaurant_id_query: &i32, feed_date_query: &NaiveDate) -> Result<Vec<Self>, diesel::result::Error> {
        let mut conn = connection().unwrap_or_else(|_| panic!("Failed to get DB connection"));
        let result = menu
            .select((restaurant_id, feed_date, time_type, menu_food, menu_price))
            .filter(restaurant_id.eq(restaurant_id_query))
            .filter(feed_date.eq(feed_date_query))
            .load::<Self>(&mut conn)?;
        Ok(result)
    }
}