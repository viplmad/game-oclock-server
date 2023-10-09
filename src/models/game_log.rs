use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::{DurationDef, Merge, ModelInfo};

#[derive(Default, Serialize, Deserialize, ToSchema)]
pub struct GameLogDTO {
    #[schema(value_type = String, format = DateTime)]
    pub start_datetime: NaiveDateTime,
    #[schema(value_type = String, format = DateTime)]
    pub end_datetime: NaiveDateTime,
    #[schema(value_type = String)]
    pub time: DurationDef,
}

impl Merge<NewGameLogDTO> for GameLogDTO {
    fn merge(self, other: NewGameLogDTO) -> Self {
        Self {
            start_datetime: other.start_datetime,
            end_datetime: other.end_datetime,
            time: self.time,
        }
    }
}

impl ModelInfo for GameLogDTO {
    const MODEL_NAME: &'static str = "Game log";
    const ID_FIELDS: &'static [&'static str] = &["game id", "datetime"];
    const UNIQUE_FIELDS: &'static [&'static str] = GameLogDTO::ID_FIELDS;
}

#[derive(Deserialize, ToSchema)]
pub struct NewGameLogDTO {
    #[schema(value_type = String, format = DateTime)]
    pub start_datetime: NaiveDateTime,
    #[schema(value_type = String, format = DateTime)]
    pub end_datetime: NaiveDateTime,
    // TODO add time
    // TODO Add optional for all
}
