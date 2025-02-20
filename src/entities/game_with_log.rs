use chrono::{NaiveDate, NaiveDateTime};
use sqlx::{FromRow, postgres::types::PgInterval};
use uuid::Uuid;

pub const LOG_START_DATETIME_ALIAS: &str = "log_start_datetime";
pub const LOG_END_DATETIME_ALIAS: &str = "log_end_datetime";
pub const LOG_DEVICE_ID_ALIAS: &str = "log_device_id";
pub const LOG_TIME_ALIAS: &str = "log_time";

#[derive(FromRow, Clone)]
pub struct GameWithLog {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub edition: String,
    pub release_date: Option<NaiveDate>,
    pub base_game_id: Option<Uuid>,
    pub cover_url: Option<String>,
    pub added_datetime: NaiveDateTime,
    pub updated_datetime: NaiveDateTime,
    pub status: i16,
    pub rating: i16,
    pub notes: String,
    pub log_start_datetime: NaiveDateTime,
    pub log_end_datetime: NaiveDateTime,
    pub log_device_id: Option<Uuid>,
    pub log_time: PgInterval,
}
