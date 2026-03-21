use std::str::FromStr;

use chrono::{DateTime, Utc};
use sea_query::enum_def;
use sqlx::{FromRow, postgres::types::PgInterval};
use uuid::Uuid;

use super::{FieldIden, FieldType, Search, TableIden};

pub const QUERY_TIME_ALIAS: &str = "query_time";
pub const SESSION_START_DATE_ALIAS: &str = "session_start_date";
pub const SESSION_END_DATE_ALIAS: &str = "session_end_date";
pub const SESSION_DEVICE_ID_ALIAS: &str = "session_device_id";
pub const SESSION_GROUP_ID_ALIAS: &str = "session_group_id";
pub const SESSION_STARTED_ALIAS: &str = "session_started";
pub const SESSION_FINISHED_STATUS_ALIAS: &str = "session_finished_status";
pub const SESSION_ADDED_DATETIME_ALIAS: &str = "session_added_datetime";
pub const SESSION_UPDATED_DATETIME_ALIAS: &str = "session_updated_datetime";

pub type SessionSearch = Search<MediaSessionIden>;

#[derive(FromRow)]
#[enum_def(table_name = "MediaSession")]
pub struct MediaSession {
    pub user_id: Uuid,
    pub media_id: Uuid,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub device_id: Option<Uuid>,
    pub group_id: Uuid,
    pub started: bool,
    pub finished_status: Option<i16>,
    pub added_datetime: DateTime<Utc>,
    pub updated_datetime: DateTime<Utc>,
}

impl TableIden for MediaSessionIden {
    const TABLE: Self = Self::Table;
}

#[derive(FromRow)]
pub struct MediaSessionWithTime {
    pub user_id: Uuid,
    pub media_id: Uuid,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub device_id: Option<Uuid>,
    pub group_id: Uuid,
    pub started: bool,
    pub finished_status: Option<i16>,
    pub query_time: PgInterval,
    pub added_datetime: DateTime<Utc>,
    pub updated_datetime: DateTime<Utc>,
}

impl FromStr for FieldIden<MediaSessionIden> {
    type Err = ();

    fn from_str(field: &str) -> Result<Self, Self::Err> {
        match field {
            "start_date" => Ok(FieldIden::new(
                MediaSessionIden::StartDate,
                FieldType::DateTime,
            )),
            "end_date" => Ok(FieldIden::new(
                MediaSessionIden::EndDate,
                FieldType::DateTime,
            )),
            "device_id" => Ok(FieldIden::new(
                MediaSessionIden::DeviceId,
                FieldType::String,
            )),
            "group_id" => Ok(FieldIden::new(MediaSessionIden::GroupId, FieldType::String)),
            "started" => Ok(FieldIden::new(
                MediaSessionIden::Started,
                FieldType::Boolean,
            )),
            "finished_status" => Ok(FieldIden::new(
                MediaSessionIden::FinishedStatus,
                FieldType::MediaStatus,
            )),
            "added_datetime" => Ok(FieldIden::new(
                MediaSessionIden::AddedDatetime,
                FieldType::DateTime,
            )),
            "updated_datetime" => Ok(FieldIden::new(
                MediaSessionIden::UpdatedDatetime,
                FieldType::DateTime,
            )),
            _ => Err(()),
        }
    }
}
