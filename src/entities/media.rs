use std::str::FromStr;

use chrono::{DateTime, FixedOffset};
use sea_query::enum_def;
use sqlx::FromRow;
use uuid::Uuid;

use super::{
    AggregateSearch, ColIden, FieldIden, FieldType, ListSearch, MediaStateIden, TableIden,
};

pub type MediaListSearch = ListSearch<MediaIden>;
pub type MediaAggregateSearch = AggregateSearch<MediaIden>;

pub const MEDIA_TYPE_GAME: &'static str = "GAME";
pub const MEDIA_TYPE_GAME_DLC: &'static str = "GAME_DLC";
pub const MEDIA_TYPE_GAME_DEMO: &'static str = "GAME_DEMO";

#[derive(FromRow)]
#[enum_def(table_name = "Media")]
pub struct Media {
    pub id: Uuid,
    pub kind: String,
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
}

impl TableIden for MediaIden {
    const TABLE: Self = Self::Table;
}

#[derive(FromRow)]
pub struct MediaWithState {
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
}

impl FromStr for FieldIden<MediaIden> {
    type Err = ();

    fn from_str(field: &str) -> Result<Self, Self::Err> {
        match field {
            "id" => Ok(FieldIden::Col(ColIden::new(
                MediaIden::Id,
                FieldType::String,
            ))),
            "kind" => Ok(FieldIden::Col(ColIden::new(
                MediaIden::Kind,
                FieldType::String,
            ))),
            "title" => Ok(FieldIden::Col(ColIden::new(
                MediaIden::Title,
                FieldType::String,
            ))),
            "edition" => Ok(FieldIden::Col(ColIden::new(
                MediaIden::Edition,
                FieldType::String,
            ))),
            "release_date" => Ok(FieldIden::Col(ColIden::new(
                MediaIden::ReleaseDate,
                FieldType::DateTime,
            ))),
            "image_url" => Ok(FieldIden::Col(ColIden::new(
                MediaIden::ImageUrl,
                FieldType::String,
            ))),
            "parent_id" => Ok(FieldIden::Col(ColIden::new(
                MediaIden::ParentId,
                FieldType::String,
            ))),
            "parent_order" => Ok(FieldIden::Col(ColIden::new(
                MediaIden::ParentOrder,
                FieldType::Integer,
            ))),
            "status" => Ok(FieldIden::Col(ColIden::new(
                MediaStateIden::Status,
                FieldType::MediaStatus,
            ))),
            "rating" => Ok(FieldIden::Col(ColIden::new(
                MediaStateIden::Rating,
                FieldType::Integer,
            ))),
            "notes" => Ok(FieldIden::Col(ColIden::new(
                MediaStateIden::Notes,
                FieldType::String,
            ))),
            "added_datetime" => Ok(FieldIden::Col(ColIden::new(
                MediaIden::AddedDatetime,
                FieldType::DateTime,
            ))),
            "updated_datetime" => Ok(FieldIden::Col(ColIden::new(
                MediaIden::UpdatedDatetime,
                FieldType::DateTime,
            ))),
            _ => Err(()),
        }
    }
}
