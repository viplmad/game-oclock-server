use chrono::{DateTime, Utc};
use sqlx::{FromRow, postgres::types::PgInterval};
use uuid::Uuid;

#[derive(FromRow, Clone)]
pub struct MediaWithStateWithSession {
    pub id: Uuid,
    pub kind: String,
    pub external_source: String,
    pub external_id: String,
    pub user_id: Uuid,
    pub title: String,
    pub edition: String,
    pub release_date: Option<DateTime<Utc>>,
    pub genres: Vec<String>,
    pub series: Vec<String>,
    pub image_url: Option<String>,
    pub parent_id: Option<Uuid>,
    pub parent_order: Option<i32>,
    pub added_datetime: DateTime<Utc>,
    pub updated_datetime: DateTime<Utc>,
    pub state_status: i16,
    pub state_rating: i16,
    pub state_notes: String,
    pub state_added_datetime: DateTime<Utc>,
    pub state_updated_datetime: DateTime<Utc>,
    // Session
    pub query_time: PgInterval,
    pub session_media_id: Uuid,
    pub session_start_date: DateTime<Utc>,
    pub session_end_date: DateTime<Utc>,
    pub session_device_id: Option<Uuid>,
    pub session_group_id: Uuid,
    pub session_started: bool,
    pub session_finished_status: Option<i16>,
    pub session_added_datetime: DateTime<Utc>,
    pub session_updated_datetime: DateTime<Utc>,
}
