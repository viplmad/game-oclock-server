use chrono::{DateTime, FixedOffset};
use sqlx::{FromRow, postgres::types::PgInterval};
use uuid::Uuid;

#[derive(FromRow)]
pub struct DeviceWithSession {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub image_url: Option<String>,
    pub added_datetime: DateTime<FixedOffset>,
    pub updated_datetime: DateTime<FixedOffset>,
    // Session
    pub query_time: PgInterval,
    pub session_media_id: Uuid,
    pub session_start_date: DateTime<FixedOffset>,
    pub session_end_date: DateTime<FixedOffset>,
    pub session_device_id: Option<Uuid>,
    pub session_group_id: Uuid,
    pub session_started: bool,
    pub session_finished_status: Option<i16>,
    pub session_added_datetime: DateTime<FixedOffset>,
    pub session_updated_datetime: DateTime<FixedOffset>,
}
