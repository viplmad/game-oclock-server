use chrono::{DateTime, Utc};
use sea_query::enum_def;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow)]
#[enum_def(table_name = "MediaSessionGroup")]
pub struct MediaSessionGroup {
    pub id: Uuid,
    pub user_id: Uuid,
    pub media_id: Uuid,
    pub name: String,
    pub added_datetime: DateTime<Utc>,
    pub updated_datetime: DateTime<Utc>,
}
