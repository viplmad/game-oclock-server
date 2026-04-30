use chrono::NaiveDate;
use sqlx::FromRow;
use uuid::Uuid;

pub const STREAK_START_DATE_ALIAS: &str = "start_date";
pub const STREAK_END_DATE_ALIAS: &str = "end_date";
pub const STREAK_DAYS_ALIAS: &str = "days";
pub const STREAK_MEDIA_IDS_ALIAS: &str = "media_ids";
pub const STREAK_DEVICE_IDS_ALIAS: &str = "device_ids";

#[derive(FromRow)]
pub struct MediaSessionStreak {
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub days: i32,
    pub media_ids: Vec<Uuid>,
    pub device_ids: Vec<Uuid>,
}
