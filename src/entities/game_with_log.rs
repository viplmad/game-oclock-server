use chrono::NaiveDateTime;
use sqlx::{postgres::types::PgInterval, FromRow};
use uuid::Uuid;

pub const LOG_DATETIME_ALIAS: &str = "log_datetime";
pub const LOG_TIME_ALIAS: &str = "log_time";

#[derive(FromRow, Clone)]
pub struct GameWithLog {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub edition: String,
    pub release_year: Option<i32>,
    pub cover_filename: Option<String>,
    pub added_datetime: NaiveDateTime,
    pub updated_datetime: NaiveDateTime,
    pub status: i16,
    pub rating: i32,
    pub notes: String,
    pub save_folder: String,
    pub screenshot_folder: String,
    pub backup: bool,
    pub log_datetime: NaiveDateTime,
    pub log_time: PgInterval,
}
