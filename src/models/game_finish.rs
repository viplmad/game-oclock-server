use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::{GameStatus, Merge, ModelInfo};

#[derive(Default, Serialize, Deserialize, ToSchema)]
pub struct GameFinishDTO {
    #[schema(value_type = String, format = Date)]
    pub date: NaiveDate,
    pub status: GameStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
}

impl Merge<NewGameFinishDTO> for GameFinishDTO {
    fn merge(self, other: NewGameFinishDTO) -> Self {
        Self {
            date: other.date,
            status: other.status,
            device_id: other.device_id,
        }
    }
}

impl ModelInfo for GameFinishDTO {
    const MODEL_NAME: &'static str = "Game finish";
    const ID_FIELDS: &'static [&'static str] = &["game id", "date"];
    const UNIQUE_FIELDS: &'static [&'static str] = GameFinishDTO::ID_FIELDS;
}

#[derive(Deserialize, ToSchema)]
pub struct NewGameFinishDTO {
    #[schema(value_type = String, format = Date)]
    pub date: NaiveDate,
    pub status: GameStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
}
