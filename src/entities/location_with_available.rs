use chrono::{DateTime, FixedOffset};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow)]
pub struct LocationWithAvailable {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub image_url: Option<String>,
    pub added_datetime: DateTime<FixedOffset>,
    pub updated_datetime: DateTime<FixedOffset>,
    pub available_date: DateTime<FixedOffset>,
    pub available_added_datetime: DateTime<FixedOffset>,
    pub available_updated_datetime: DateTime<FixedOffset>,
}
