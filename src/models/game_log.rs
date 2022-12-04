use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::{DurationDef, ModelInfo};

#[derive(Serialize, Deserialize, ToSchema)]
pub struct GameLogDTO {
    #[schema(value_type = String)]
    pub datetime: NaiveDateTime,
    #[schema(value_type = String)]
    pub time: DurationDef,
}

impl ModelInfo for GameLogDTO {
    const MODEL_NAME: &'static str = "Game log";
    const ID_FIELDS: &'static [&'static str] = &["game id", "datetime"];
    const UNIQUE_FIELDS: &'static [&'static str] = GameLogDTO::ID_FIELDS;
}
