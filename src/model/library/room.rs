use crate::db::connection;
use crate::schema::reading_room::dsl::*;
use diesel::prelude::*;

#[derive(Queryable)]
pub struct ReadingRoomItem {
    #[diesel(sql_type = Integer)]
    pub campus_id: i32,
    #[diesel(sql_type = Integer)]
    pub room_id: i32,
    #[diesel(sql_type = Text)]
    pub room_name: String,
    #[diesel(sql_type = Bool)]
    pub is_active: bool,
    #[diesel(sql_type = Bool)]
    pub is_reservable: bool,
    #[diesel(sql_type = Integer)]
    pub total: i32,
    #[diesel(sql_type = Integer)]
    pub active_total: i32,
    #[diesel(sql_type = Integer)]
    pub occupied: i32,
    #[diesel(sql_type = Integer)]
    pub available: i32,
}

impl ReadingRoomItem {
    pub fn find_by_campus_id(campus_id_query: &i32) -> Result<Vec<Self>, diesel::result::Error> {
        let mut conn = connection().unwrap_or_else(|_| panic!("Failed to get DB connection"));
        let result = reading_room
            .filter(campus_id.eq(campus_id_query))
            .load::<Self>(&mut conn)?;
        Ok(result)
    }

    pub fn get_by_id(
        campus_id_query: &i32,
        room_id_query: &i32,
    ) -> Result<Self, diesel::result::Error> {
        let mut conn = connection().unwrap_or_else(|_| panic!("Failed to get DB connection"));
        let result = reading_room
            .filter(campus_id.eq(campus_id_query))
            .filter(room_id.eq(room_id_query))
            .first::<Self>(&mut conn)?;
        Ok(result)
    }
}
