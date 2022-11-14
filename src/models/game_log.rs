use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::{DurationDef, ModelName};

#[derive(Serialize, Deserialize, ToSchema)]
pub struct GameLogDTO {
    pub datetime: NaiveDateTime,
    pub time: DurationDef,
}

impl ModelName for GameLogDTO {
    const MODEL_NAME: &'static str = "Game log";
    const ID_FIELDS: &'static [&'static str] = &["game id", "datetime"];
    const UNIQUE_FIELDS: &'static [&'static str] = GameLogDTO::ID_FIELDS;
}
