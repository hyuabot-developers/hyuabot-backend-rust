use chrono::NaiveDate;
use diesel::prelude::*;

use crate::db::connection;
use crate::schema::menu::dsl::*;
use crate::schema::menu::dsl as menu_table;
use crate::schema::restaurant::dsl::*;


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

#[derive(Queryable)]
pub struct RestaurantMenuItem {
    #[diesel(sql_type = Integer)]
    pub restaurant_id: i32,
    #[diesel(sql_type = Text)]
    pub restaurant_name: String,
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
            .select((menu_table::restaurant_id, feed_date, time_type, menu_food, menu_price))
            .filter(menu_table::restaurant_id.eq(restaurant_id_query))
            .filter(feed_date.eq(feed_date_query))
            .load::<Self>(&mut conn)?;
        Ok(result)
    }
}

impl RestaurantMenuItem {
    pub fn find_by_campus_id_and_time(campus_id_query: &i32, feed_date_query: &NaiveDate, time_type_query: &str) -> Result<Vec<Self>, diesel::result::Error> {
        let mut conn = connection().unwrap_or_else(|_| panic!("Failed to get DB connection"));
        let result = menu
            .inner_join(restaurant)
            .select((menu_table::restaurant_id, restaurant_name, feed_date, time_type, menu_food, menu_price))
            .filter(campus_id.eq(campus_id_query))
            .filter(feed_date.eq(feed_date_query))
            .filter(time_type.like(format!("%{}%", time_type_query)))
            .load::<Self>(&mut conn)?;
        Ok(result)
    }
}