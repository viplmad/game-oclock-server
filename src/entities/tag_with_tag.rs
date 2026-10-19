use chrono::{DateTime, FixedOffset};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow)]
pub struct TagWithTag {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub added_datetime: DateTime<FixedOffset>,
    pub updated_datetime: DateTime<FixedOffset>,
    pub tag_order: Option<i32>,
    pub tag_added_datetime: DateTime<FixedOffset>,
    pub tag_updated_datetime: DateTime<FixedOffset>,
}
