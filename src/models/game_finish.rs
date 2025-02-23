use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use super::{GameStatus, Merge, ModelInfo};

#[derive(Default, Serialize, Deserialize, ToSchema)]
pub struct FinishDTO {
    #[schema(value_type = String, format = DateTime)]
    pub datetime: DateTime<Utc>,
    pub status: GameStatus, // TODO use Finish Status
    #[schema(value_type = String)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<Uuid>,
}

impl Merge<NewFinishDTO> for FinishDTO {
    fn merge(self, other: NewFinishDTO) -> Self {
        Self {
            datetime: other.datetime,
            status: other.status,
            device_id: other.device_id,
        }
    }
}

impl ModelInfo for FinishDTO {
    const MODEL_NAME: &'static str = "Game finish";
    const ID_FIELDS: &'static [&'static str] = &["game id", "date"];
    const UNIQUE_FIELDS: &'static [&'static str] = FinishDTO::ID_FIELDS;
}

#[derive(Deserialize, ToSchema)]
pub struct NewFinishDTO {
    #[schema(value_type = String, format = DateTime)]
    pub datetime: DateTime<Utc>,
    pub status: GameStatus,
    #[schema(value_type = String)]
    pub device_id: Option<Uuid>,
}
