use serde::Serialize;
use utoipa::ToSchema;

use super::{FinishDTO, GameDTO, ModelInfo};

#[derive(Serialize, ToSchema)]
pub struct GameWithFinishDTO {
    pub game: GameDTO,
    pub finish: FinishDTO,
}

impl ModelInfo for GameWithFinishDTO {
    const MODEL_NAME: &'static str = "Game with finish";
    const ID_FIELDS: &'static [&'static str] = GameDTO::ID_FIELDS;
    const UNIQUE_FIELDS: &'static [&'static str] = GameDTO::UNIQUE_FIELDS;
}
