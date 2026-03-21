use chrono::{DateTime, Utc};
use sea_query::enum_def;
use sqlx::FromRow;
use uuid::Uuid;

use super::TableIden;

pub const STATE_STATUS_ALIAS: &str = "state_status";
pub const STATE_RATING_ALIAS: &str = "state_rating";
pub const STATE_NOTES_ALIAS: &str = "state_notes";
pub const STATE_ADDED_DATETIME_ALIAS: &str = "state_added_datetime";
pub const STATE_UPDATED_DATETIME_ALIAS: &str = "state_updated_datetime";

#[derive(FromRow)]
#[enum_def(table_name = "MediaState")]
pub struct MediaState {
    pub user_id: Uuid,
    pub media_id: Uuid,
    pub status: i16,
    pub rating: i16,
    pub notes: String,
    pub added_datetime: DateTime<Utc>,
    pub updated_datetime: DateTime<Utc>,
}

impl TableIden for MediaStateIden {
    const TABLE: Self = Self::Table;
}
