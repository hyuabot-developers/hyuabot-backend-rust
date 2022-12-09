use chrono::Datelike;

pub fn get_subway_weekday() -> String {
    if chrono::Local::now().weekday() == chrono::Weekday::Sat || chrono::Local::now().weekday() == chrono::Weekday::Sun {
        "weekends".to_string()
    } else {
        "weekdays".to_string()
    }
}