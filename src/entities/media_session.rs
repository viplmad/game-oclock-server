use std::str::FromStr;

use chrono::{DateTime, FixedOffset};
use sea_query::{Expr, enum_def};
use sqlx::{FromRow, postgres::types::PgInterval};
use uuid::Uuid;

use crate::entities::MediaIden;

use super::{
    AggregateGroupSearch, AggregateSearch, ColIden, ExprIden, FieldIden, FieldType, ListSearch,
    MediaStateIden, TableIden,
};

pub const QUERY_TIME_ALIAS: &str = "query_time";
pub const SESSION_MEDIA_ID_ALIAS: &str = "session_media_id";
pub const SESSION_START_DATE_ALIAS: &str = "session_start_date";
pub const SESSION_END_DATE_ALIAS: &str = "session_end_date";
pub const SESSION_DEVICE_ID_ALIAS: &str = "session_device_id";
pub const SESSION_GROUP_ID_ALIAS: &str = "session_group_id";
pub const SESSION_STARTED_ALIAS: &str = "session_started";
pub const SESSION_FINISHED_STATUS_ALIAS: &str = "session_finished_status";
pub const SESSION_ADDED_DATETIME_ALIAS: &str = "session_added_datetime";
pub const SESSION_UPDATED_DATETIME_ALIAS: &str = "session_updated_datetime";

pub type SessionListSearch = ListSearch<MediaSessionIden>;
pub type SessionAggregateSearch = AggregateSearch<MediaSessionIden>;
pub type SessionAggregateGroupSearch = AggregateGroupSearch<MediaSessionIden>;

#[derive(FromRow)]
#[enum_def(table_name = "MediaSession")]
pub struct MediaSession {
    pub user_id: Uuid,
    pub media_id: Uuid,
    pub start_date: DateTime<FixedOffset>,
    pub start_date_tz: String,
    pub end_date: DateTime<FixedOffset>,
    pub end_date_tz: String,
    pub device_id: Option<Uuid>,
    pub group_id: Uuid,
    pub started: bool,
    pub finished_status: Option<i16>,
    pub added_datetime: DateTime<FixedOffset>,
    pub updated_datetime: DateTime<FixedOffset>,
}

impl TableIden for MediaSessionIden {
    const TABLE: Self = Self::Table;
}

#[derive(FromRow)]
pub struct MediaSessionWithTime {
    pub user_id: Uuid,
    pub media_id: Uuid,
    pub start_date: DateTime<FixedOffset>,
    pub end_date: DateTime<FixedOffset>,
    pub device_id: Option<Uuid>,
    pub group_id: Uuid,
    pub started: bool,
    pub finished_status: Option<i16>,
    pub query_time: PgInterval,
    pub added_datetime: DateTime<FixedOffset>,
    pub updated_datetime: DateTime<FixedOffset>,
}

impl FromStr for FieldIden<MediaSessionIden> {
    type Err = ();

    fn from_str(field: &str) -> Result<Self, Self::Err> {
        match field {
            "time" => Ok(FieldIden::Expr(ExprIden::new::<MediaSessionIden>(
                Expr::col((MediaSessionIden::Table, MediaSessionIden::EndDate)).sub(Expr::col((
                    MediaSessionIden::Table,
                    MediaSessionIden::StartDate,
                ))),
                FieldType::Interval,
            ))),
            "media_id" => Ok(FieldIden::Col(ColIden::new(
                MediaSessionIden::MediaId,
                FieldType::String,
            ))),
            "start_date" => Ok(FieldIden::Col(ColIden::new(
                MediaSessionIden::StartDate,
                FieldType::DateTime,
            ))),
            "end_date" => Ok(FieldIden::Col(ColIden::new(
                MediaSessionIden::EndDate,
                FieldType::DateTime,
            ))),
            "device_id" => Ok(FieldIden::Col(ColIden::new(
                MediaSessionIden::DeviceId,
                FieldType::String,
            ))),
            "group_id" => Ok(FieldIden::Col(ColIden::new(
                MediaSessionIden::GroupId,
                FieldType::String,
            ))),
            "started" => Ok(FieldIden::Col(ColIden::new(
                MediaSessionIden::Started,
                FieldType::Boolean,
            ))),
            "finished_status" => Ok(FieldIden::Col(ColIden::new(
                MediaSessionIden::FinishedStatus,
                FieldType::MediaStatus,
            ))),
            "added_datetime" => Ok(FieldIden::Col(ColIden::new(
                MediaSessionIden::AddedDatetime,
                FieldType::DateTime,
            ))),
            "updated_datetime" => Ok(FieldIden::Col(ColIden::new(
                MediaSessionIden::UpdatedDatetime,
                FieldType::DateTime,
            ))),
            "media_release_date" => Ok(FieldIden::ExtCol(ColIden::new(
                MediaIden::ReleaseDate,
                FieldType::DateTime,
            ))),
            "media_rating" => Ok(FieldIden::ExtCol(ColIden::new(
                MediaStateIden::Rating,
                FieldType::Integer,
            ))),
            _ => Err(()),
        }
    }
}
