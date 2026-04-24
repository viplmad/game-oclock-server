use chrono::{DateTime, FixedOffset, NaiveDate, NaiveTime, Utc};

pub const MICROS_PER_SECOND: i64 = 1_000_000;
pub const SECONDS_PER_MINUTE: i64 = 60;
pub const MINUTES_PER_HOUR: i64 = 60;
pub const HOURS_PER_DAY: i64 = 24;
pub const SECONDS_PER_HOUR: i64 = MINUTES_PER_HOUR * SECONDS_PER_MINUTE;
pub const SECONDS_PER_DAY: i64 = HOURS_PER_DAY * SECONDS_PER_HOUR;

pub fn now() -> DateTime<FixedOffset> {
    Utc::now().fixed_offset()
}

pub fn date_at_start_of_day(date: NaiveDate) -> DateTime<FixedOffset> {
    let start_of_day_time = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
    date.and_time(start_of_day_time).and_utc().fixed_offset()
}
