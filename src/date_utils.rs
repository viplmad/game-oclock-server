use chrono::{DateTime, NaiveDate, NaiveTime, Utc};

pub const MICROS_PER_SECOND: i64 = 1_000_000;
pub const SECONDS_PER_MINUTE: i64 = 60;
pub const MINUTES_PER_HOUR: i64 = 60;
pub const HOURS_PER_DAY: i64 = 24;
pub const SECONDS_PER_HOUR: i64 = MINUTES_PER_HOUR * SECONDS_PER_MINUTE;
pub const SECONDS_PER_DAY: i64 = HOURS_PER_DAY * SECONDS_PER_HOUR;

pub fn now() -> DateTime<Utc> {
    chrono::Utc::now()
}

pub fn date_at_start_of_day(date: NaiveDate) -> DateTime<Utc> {
    let start_of_day_time = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
    date.and_time(start_of_day_time).and_utc()
}

pub fn date_at_midnight(date: NaiveDate) -> DateTime<Utc> {
    let midnight_time = NaiveTime::from_hms_opt(23, 59, 59).unwrap();
    date.and_time(midnight_time).and_utc()
}
