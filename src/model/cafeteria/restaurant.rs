use diesel::prelude::*;

use crate::db::connection;
use crate::schema::restaurant::dsl::*;

#[derive(Queryable)]
pub struct RestaurantItem {
    #[diesel(sql_type = Integer)]
    pub campus_id: i32,
    #[diesel(sql_type = Integer)]
    pub restaurant_id: i32,
    #[diesel(sql_type = Text)]
    pub restaurant_name: String,
}

impl RestaurantItem {
    pub fn find_by_campus_id(campus_id_query: &i32) -> Result<Vec<Self>, diesel::result::Error> {
        let mut conn = connection().unwrap_or_else(|_| panic!("Failed to get DB connection"));
        let result = restaurant
            .select((campus_id, restaurant_id, restaurant_name))
            .filter(campus_id.eq(campus_id_query))
            .load::<Self>(&mut conn)?;
        Ok(result)
    }

    pub fn get_by_id(
        campus_id_query: &i32,
        restaurant_id_query: &i32,
    ) -> Result<Self, diesel::result::Error> {
        let mut conn = connection().unwrap_or_else(|_| panic!("Failed to get DB connection"));
        let result = restaurant
            .select((campus_id, restaurant_id, restaurant_name))
            .filter(campus_id.eq(campus_id_query))
            .filter(restaurant_id.eq(restaurant_id_query))
            .first::<Self>(&mut conn)?;
        Ok(result)
    }
}
