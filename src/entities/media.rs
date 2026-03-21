use std::str::FromStr;

use chrono::{DateTime, Utc};
use sea_query::enum_def;
use sqlx::FromRow;
use uuid::Uuid;

use super::{FieldIden, FieldType, MediaStateIden, Search, TableIden};

pub type MediaSearch = Search<MediaIden>;

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
    pub release_date: Option<DateTime<Utc>>,
    pub genres: Vec<String>,
    pub series: Vec<String>,
    pub image_url: Option<String>,
    pub parent_id: Option<Uuid>,
    pub parent_order: Option<i32>,
    pub added_datetime: DateTime<Utc>,
    pub updated_datetime: DateTime<Utc>,
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
}

impl FromStr for FieldIden<MediaIden> {
    type Err = ();

    fn from_str(field: &str) -> Result<Self, Self::Err> {
        match field {
            "id" => Ok(FieldIden::new(MediaIden::Id, FieldType::String)),
            "title" => Ok(FieldIden::new(MediaIden::Title, FieldType::String)),
            "edition" => Ok(FieldIden::new(MediaIden::Edition, FieldType::String)),
            "release_date" => Ok(FieldIden::new(MediaIden::ReleaseDate, FieldType::Integer)),
            "image_url" => Ok(FieldIden::new(MediaIden::ImageUrl, FieldType::String)),
            "parent_id" => Ok(FieldIden::new(MediaIden::ParentId, FieldType::String)),
            "parent_order" => Ok(FieldIden::new(MediaIden::ParentOrder, FieldType::Integer)),
            "status" => Ok(FieldIden::new(
                MediaStateIden::Status,
                FieldType::MediaStatus,
            )),
            "rating" => Ok(FieldIden::new(MediaStateIden::Rating, FieldType::Integer)),
            "notes" => Ok(FieldIden::new(MediaStateIden::Notes, FieldType::String)),
            "added_datetime" => Ok(FieldIden::new(
                MediaIden::AddedDatetime,
                FieldType::DateTime,
            )),
            "updated_datetime" => Ok(FieldIden::new(
                MediaIden::UpdatedDatetime,
                FieldType::DateTime,
            )),
            _ => Err(()),
        }
    }
}
