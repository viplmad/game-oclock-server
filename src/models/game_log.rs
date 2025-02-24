use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use super::{DurationDef, Merge, ModelInfo};

#[derive(Default, Clone, Serialize, Deserialize, ToSchema)]
pub struct LogDTO {
    #[schema(value_type = String, format = DateTime)]
    pub start_datetime: DateTime<Utc>,
    #[schema(value_type = String, format = DateTime)]
    pub end_datetime: DateTime<Utc>,
    #[schema(value_type = String)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<Uuid>,
    #[schema(value_type = String)]
    pub time: DurationDef,
}

impl Merge<NewLogDTO> for LogDTO {
    fn merge(self, other: NewLogDTO) -> Self {
        Self {
            start_datetime: other.start_datetime,
            end_datetime: other.end_datetime,
            device_id: other.device_id,
            time: self.time,
        }
    }
}

impl ModelInfo for LogDTO {
    const MODEL_NAME: &'static str = "Game log";
    const ID_FIELDS: &'static [&'static str] = &["game id", "start datetime"];
    const UNIQUE_FIELDS: &'static [&'static str] = LogDTO::ID_FIELDS;
}

#[derive(Deserialize, ToSchema)]
pub struct NewLogDTO {
    #[schema(value_type = String, format = DateTime)]
    pub start_datetime: DateTime<Utc>,
    #[schema(value_type = String, format = DateTime)]
    pub end_datetime: DateTime<Utc>,
    #[schema(value_type = String)]
    pub device_id: Option<Uuid>,
}
