use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow)]
pub struct LocationWithAvailable {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub image_url: Option<String>,
    pub added_datetime: DateTime<Utc>,
    pub updated_datetime: DateTime<Utc>,
    pub available_date: DateTime<Utc>,
    pub available_added_datetime: DateTime<Utc>,
    pub available_updated_datetime: DateTime<Utc>,
}
