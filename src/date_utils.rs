use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

pub fn now() -> NaiveDateTime {
    chrono::Utc::now().naive_utc()
}

pub fn date_at_start_of_day(date: NaiveDate) -> NaiveDateTime {
    let start_of_day_time = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
    date.and_time(start_of_day_time)
}

pub fn date_at_midnight(date: NaiveDate) -> NaiveDateTime {
    let midnight_time = NaiveTime::from_hms_opt(23, 59, 59).unwrap();
    date.and_time(midnight_time)
}
