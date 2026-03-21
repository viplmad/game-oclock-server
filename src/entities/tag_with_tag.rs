use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow)]
pub struct TagWithTag {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub added_datetime: DateTime<Utc>,
    pub updated_datetime: DateTime<Utc>,
    pub tag_order: i32,
    pub tag_added_datetime: DateTime<Utc>,
    pub tag_updated_datetime: DateTime<Utc>,
}
