use chrono::NaiveDateTime;
use sqlx::{postgres::types::PgInterval, FromRow};

#[derive(FromRow, Clone)]
pub struct GameWithLog {
    pub id: i32,
    pub user_id: i32,
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
    pub datetime: NaiveDateTime,
    pub time: PgInterval,
}
