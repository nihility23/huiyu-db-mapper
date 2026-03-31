use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, TimeZone};
use chrono::LocalResult::Single;
use crate::base::error::DatabaseError;

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

pub fn format_date_time_local(time_local: &DateTime<Local>, time_format: &str) -> String {
    time_local.format(time_format).to_string()
}

pub fn create_datetime_local_from_seconds(seconds: i64)-> DateTime<Local> {
    // 转换为 NaiveDateTime（无时区）
    let utc = DateTime::from_timestamp(seconds, 0).unwrap();
    // 转换为本地时间
    let local: DateTime<Local> = DateTime::from(utc);
    local
}

pub fn format_date_time_local_from_str(time_str: &str, time_format: &str) -> Result<DateTime<Local>,DatabaseError> {
    let date = NaiveDateTime::parse_from_str(time_str, time_format).map_err(|e| DatabaseError::ConvertError(e.to_string()))?;
    let local_date_time = Local.from_local_datetime(&date);
    match local_date_time {
        Single(v) => Ok(v),
        _ => Err(DatabaseError::ConvertError("Invalid datetime".to_string())),
    }
}