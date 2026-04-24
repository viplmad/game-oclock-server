use chrono::{DateTime, FixedOffset};
use sea_query::enum_def;
use sqlx::FromRow;
use uuid::Uuid;

pub const AVAILABLE_DATE_ALIAS: &str = "available_date";
pub const AVAILABLE_ADDED_DATETIME_ALIAS: &str = "available_added_datetime";
pub const AVAILABLE_UPDATED_DATETIME_ALIAS: &str = "available_updated_datetime";

#[derive(FromRow)]
#[enum_def(table_name = "MediaAvailable")]
pub struct MediaAvailable {
    pub user_id: Uuid,
    pub media_id: Uuid,
    pub location_id: Uuid,
    pub date: DateTime<FixedOffset>,
    pub added_datetime: DateTime<FixedOffset>,
    pub updated_datetime: DateTime<FixedOffset>,
}
