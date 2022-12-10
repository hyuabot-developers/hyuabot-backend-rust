use chrono::NaiveDate;
use diesel::prelude::*;

use crate::db::connection;
use crate::schema::shuttle_period::dsl::*;
use crate::schema::shuttle_holiday::dsl::*;


#[derive(Queryable)]
pub struct ShuttlePeriodItem {
    #[diesel(sql_type = Text)]
    pub period_type: String,
    #[diesel(sql_type = Timestamp)]
    pub period_start: chrono::NaiveDateTime,
    #[diesel(sql_type = Timestamp)]
    pub period_end: chrono::NaiveDateTime,
}

#[derive(Queryable)]
pub struct ShuttleHolidayItem {
    #[diesel(sql_type = Date)]
    pub holiday_date: NaiveDate,
    #[diesel(sql_type = Text)]
    pub holiday_type: String,
    #[diesel(sql_type = Text)]
    pub calendar_type: String,
}

impl ShuttlePeriodItem {
    pub fn get_current_period() -> Result<Self, diesel::result::Error> {
        let mut conn = connection().unwrap_or_else(|_| panic!("Failed to get DB connection"));
        let current_time = chrono::Utc::now().naive_local();
        let period = shuttle_period
            .filter(period_start.lt(current_time))
            .filter(period_end.gt(current_time))
            .first::<ShuttlePeriodItem>(&mut conn)?;
        Ok(period)
    }
}

impl ShuttleHolidayItem {
    pub fn get_holiday_by_date(date: chrono::NaiveDateTime) -> Result<Self, diesel::result::Error> {
        let mut conn = connection().unwrap_or_else(|_| panic!("Failed to get DB connection"));
        let holiday = shuttle_holiday
            .filter(holiday_date.eq(date.date()))
            .first::<ShuttleHolidayItem>(&mut conn)?;
        Ok(holiday)
    }
}