use chrono::{DateTime, FixedOffset};
use sea_query::enum_def;
use sqlx::FromRow;
use uuid::Uuid;

pub const TAG_ORDER_ALIAS: &str = "tag_order";
pub const TAG_ADDED_DATETIME_ALIAS: &str = "tag_added_datetime";
pub const TAG_UPDATED_DATETIME_ALIAS: &str = "tag_updated_datetime";

#[derive(FromRow)]
#[enum_def(table_name = "MediaTag")]
pub struct MediaTag {
    pub user_id: Uuid,
    pub media_id: Uuid,
    pub tag_id: Uuid,
    pub order: i32,
    pub added_datetime: DateTime<FixedOffset>,
    pub updated_datetime: DateTime<FixedOffset>,
}
