use chrono::{DateTime, FixedOffset};
use sea_query::enum_def;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow)]
#[enum_def(table_name = "StoredResponse")]
pub struct StoredResponse {
    pub user_id: Uuid,
    pub scope: String,
    pub request_hash: String,
    pub response: String,
    pub last_used_date: DateTime<FixedOffset>,
    pub added_datetime: DateTime<FixedOffset>,
    pub updated_datetime: DateTime<FixedOffset>,
}
