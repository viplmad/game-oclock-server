use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use super::{ExternalMediaIdDTO, MediaStatus, MediaType, Merge, ModelInfo};

#[derive(Default, Serialize, ToSchema)]
pub struct MediaDTO {
    pub media: MediaDataDTO,
    pub external: ExternalMediaIdDTO,
    pub state: MediaStateDTO,
}

#[derive(Default, Serialize, ToSchema)]
pub struct PotentialMediaDTO {
    pub media: ExternalMediaDataDTO,
    pub external: ExternalMediaIdDTO,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<MediaStateDTO>,
}

#[derive(Default, Serialize, ToSchema)]
pub struct MediaDataDTO {
    pub id: Uuid,
    pub kind: MediaType,
    pub title: String,
    pub edition: String,
    #[schema(value_type = String, format = DateTime)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_date: Option<DateTime<FixedOffset>>,
    pub genres: Vec<String>,
    pub series: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_order: Option<u32>,
    #[schema(value_type = String, format = DateTime)]
    pub added_datetime: DateTime<FixedOffset>,
    #[schema(value_type = String, format = DateTime)]
    pub updated_datetime: DateTime<FixedOffset>,
}

#[derive(Default, Serialize, ToSchema)]
pub struct ExternalMediaDataDTO {
    pub id: Option<Uuid>,
    pub kind: MediaType,
    pub title: String,
    pub edition: String,
    #[schema(value_type = String, format = DateTime)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_date: Option<DateTime<FixedOffset>>,
    pub genres: Vec<String>,
    pub series: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_order: Option<u32>,
}

#[derive(Default, Serialize, ToSchema)]
pub struct MediaStateDTO {
    pub status: MediaStatus,
    pub rating: u32,
    pub notes: String,
    #[schema(value_type = String, format = DateTime)]
    pub added_datetime: DateTime<FixedOffset>,
    #[schema(value_type = String, format = DateTime)]
    pub updated_datetime: DateTime<FixedOffset>,
}

impl Merge<NewManualMediaDTO> for MediaDataDTO {
    fn merge(self, other: NewManualMediaDTO) -> Self {
        Self {
            id: self.id,
            title: other.title.unwrap_or(self.title),
            edition: other.edition.unwrap_or(self.edition),
            release_date: other.release_date,
            genres: other.genres,
            series: other.series,
            image_url: other.image_url,
            kind: other.kind.unwrap_or(self.kind),
            parent_id: other.parent_id,
            parent_order: other.parent_order,
            added_datetime: self.added_datetime,
            updated_datetime: self.updated_datetime,
        }
    }
}

impl Merge<NewMediaStateDTO> for MediaStateDTO {
    fn merge(self, other: NewMediaStateDTO) -> Self {
        Self {
            status: other.status.unwrap_or(self.status),
            rating: other.rating.unwrap_or(self.rating),
            notes: other.notes.unwrap_or(self.notes),
            added_datetime: self.added_datetime,
            updated_datetime: self.updated_datetime,
        }
    }
}

impl ModelInfo for MediaDTO {
    const MODEL_NAME: &'static str = "Media";
    const ID_FIELDS: &'static [&'static str] = &["id"];
    const UNIQUE_FIELDS: &'static [&'static str] = &["kind", "title", "edition"];
}

impl ModelInfo for MediaDataDTO {
    const MODEL_NAME: &'static str = "Media";
    const ID_FIELDS: &'static [&'static str] = &["id"];
    const UNIQUE_FIELDS: &'static [&'static str] = &["kind", "title", "edition"];
}

impl ModelInfo for MediaStateDTO {
    const MODEL_NAME: &'static str = "MediaState";
    const ID_FIELDS: &'static [&'static str] = &["user_id", "media_id"];
    const UNIQUE_FIELDS: &'static [&'static str] = &["user_id", "media_id"];
}

#[derive(Deserialize, ToSchema)]
pub struct NewMediaDTO {
    pub media: NewMediaValue,
    pub state: NewMediaStateDTO,
}

#[derive(Deserialize, ToSchema)]
#[serde(untagged)]
pub enum NewMediaValue {
    Manual(NewManualMediaDTO),
    External(ExternalMediaIdDTO),
}

#[derive(Deserialize, ToSchema)]
pub struct NewMediaStateDTO {
    pub status: Option<MediaStatus>,
    pub rating: Option<u32>,
    pub notes: Option<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct NewManualMediaDTO {
    pub kind: Option<MediaType>,
    pub title: Option<String>,
    pub edition: Option<String>,
    pub release_date: Option<DateTime<FixedOffset>>,
    pub genres: Vec<String>,
    pub series: Vec<String>,
    pub image_url: Option<String>,
    pub parent_id: Option<Uuid>,
    pub parent_order: Option<u32>,
}
