use chrono::NaiveDate;
use sqlx::FromRow;

pub const STREAK_START_DATE_ALIAS: &str = "start_date";
pub const STREAK_END_DATE_ALIAS: &str = "end_date";
pub const STREAK_DAYS_ALIAS: &str = "days";

#[derive(FromRow)]
pub struct MediaSessionStreak {
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub days: i32,
}
