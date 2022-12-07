use chrono::Datelike;
use crate::model::shuttle::period::ShuttleHolidayItem;

pub fn get_shuttle_weekday() -> String {
    match ShuttleHolidayItem::get_holiday_by_date(chrono::Local::now().naive_local()) {
        Ok(holiday_item) => holiday_item.holiday_type,
        Err(_) => {
            if chrono::Local::now().weekday() == chrono::Weekday::Sat || chrono::Local::now().weekday() == chrono::Weekday::Sun {
                "weekends".to_string()
            } else {
                "weekdays".to_string()
            }
        }
    }
}