use chrono::{DateTime, FixedOffset};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow)]
pub struct MediaWithStateWithTag {
    pub id: Uuid,
    pub kind: String,
    pub external_source: String,
    pub external_id: String,
    pub user_id: Uuid,
    pub title: String,
    pub edition: String,
    pub release_date: Option<DateTime<FixedOffset>>,
    pub genres: Vec<String>,
    pub series: Vec<String>,
    pub image_url: Option<String>,
    pub parent_id: Option<Uuid>,
    pub parent_order: Option<i32>,
    pub added_datetime: DateTime<FixedOffset>,
    pub updated_datetime: DateTime<FixedOffset>,
    pub state_status: i16,
    pub state_rating: i16,
    pub state_notes: String,
    pub state_added_datetime: DateTime<FixedOffset>,
    pub state_updated_datetime: DateTime<FixedOffset>,
    pub tag_order: Option<i32>,
    pub tag_added_datetime: DateTime<FixedOffset>,
    pub tag_updated_datetime: DateTime<FixedOffset>,
}
