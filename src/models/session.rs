use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use super::{DurationDef, MediaStatus, Merge, ModelInfo};

#[derive(Default, Clone, Serialize, Deserialize, ToSchema)]
pub struct SessionDTO {
    #[schema(value_type = String)]
    pub media_id: Uuid,
    #[schema(value_type = String, format = DateTime)]
    pub start_datetime: DateTime<Utc>,
    #[schema(value_type = String, format = DateTime)]
    pub end_datetime: DateTime<Utc>,
    #[schema(value_type = String)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<Uuid>,
    #[schema(value_type = String)]
    pub group_id: Uuid,
    pub started: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished_status: Option<MediaStatus>,
    #[schema(value_type = String)]
    pub time: DurationDef,
    #[schema(value_type = String, format = DateTime)]
    pub added_datetime: DateTime<Utc>,
    #[schema(value_type = String, format = DateTime)]
    pub updated_datetime: DateTime<Utc>,
}

impl Merge<NewSessionDTO> for SessionDTO {
    fn merge(self, other: NewSessionDTO) -> Self {
        Self {
            media_id: self.media_id,
            start_datetime: other.start_datetime,
            end_datetime: other.end_datetime,
            device_id: other.device_id,
            group_id: other.group_id.unwrap_or(self.group_id),
            started: other.started,
            finished_status: other.finished_status,
            time: self.time,
            added_datetime: self.added_datetime,
            updated_datetime: self.updated_datetime,
        }
    }
}

impl ModelInfo for SessionDTO {
    const MODEL_NAME: &'static str = "Media session";
    const ID_FIELDS: &'static [&'static str] = &["media id", "start datetime"];
    const UNIQUE_FIELDS: &'static [&'static str] = SessionDTO::ID_FIELDS;
}

#[derive(Deserialize, ToSchema)]
pub struct NewSessionDTO {
    #[schema(value_type = String, format = DateTime)]
    pub start_datetime: DateTime<Utc>,
    #[schema(value_type = String, format = DateTime)]
    pub end_datetime: DateTime<Utc>,
    #[schema(value_type = String)]
    pub device_id: Option<Uuid>,
    #[schema(value_type = String)]
    pub group_id: Option<Uuid>,
    pub started: bool,
    pub finished_status: Option<MediaStatus>,
}
