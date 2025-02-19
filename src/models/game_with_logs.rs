use serde::Serialize;
use utoipa::ToSchema;

use super::{GameDTO, LogDTO, ModelInfo};

#[derive(Serialize, ToSchema)]
pub struct GameWithLogDTO {
    pub game: GameDTO,
    pub log: LogDTO,
}

impl ModelInfo for GameWithLogDTO {
    const MODEL_NAME: &'static str = "Game with log";
    const ID_FIELDS: &'static [&'static str] = GameDTO::ID_FIELDS;
    const UNIQUE_FIELDS: &'static [&'static str] = GameDTO::UNIQUE_FIELDS;
}
