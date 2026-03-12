use std::time;
use chrono::{DateTime, Local, NaiveDate};

pub fn create_datetime_local(
    year: i32, month: u32, day: u32,
    hour: u32, minute: u32, second: u32, millisecond: u32
) -> DateTime<Local> {
    NaiveDate::from_ymd_opt(year, month, day)
        .unwrap()
        .and_hms_milli_opt(hour, minute, second, millisecond)
        .unwrap()
        .and_local_timezone(Local)
        .unwrap()
}
