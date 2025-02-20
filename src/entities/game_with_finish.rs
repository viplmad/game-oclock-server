use chrono::{DateTime, NaiveDate, Utc};
use sqlx::FromRow;
use uuid::Uuid;

pub const FINISH_DATE_ALIAS: &str = "finish_date";
pub const FINISH_STATUS_ALIAS: &str = "finish_status";
pub const FINISH_DEVICE_ID_ALIAS: &str = "finish_device_id";

#[derive(FromRow, Clone)]
pub struct GameWithFinish {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub edition: String,
    pub release_date: Option<NaiveDate>,
    pub base_game_id: Option<Uuid>,
    pub cover_url: Option<String>,
    pub added_datetime: DateTime<Utc>,
    pub updated_datetime: DateTime<Utc>,
    pub status: i16,
    pub rating: i16,
    pub notes: String,
    pub finish_date: NaiveDate,
    pub finish_status: i16,
    pub finish_device_id: Option<Uuid>,
}
