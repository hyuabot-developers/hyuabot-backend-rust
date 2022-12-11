use chrono::Datelike;

pub fn get_bus_weekday() -> String {
    if chrono::Local::now().weekday() == chrono::Weekday::Sat {
        "saturday".to_string()
    } else if chrono::Local::now().weekday() == chrono::Weekday::Sun {
        "sunday".to_string()
    } else {
        "weekdays".to_string()
    }
}
