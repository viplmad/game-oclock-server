use chrono::NaiveDate;
use serde::Serialize;
use utoipa::ToSchema;

use super::{GameDTO, ModelInfo};

#[derive(Serialize, ToSchema)]
pub struct GameAvailableDTO {
    pub game: GameDTO,
    #[schema(value_type = String, format = Date)]
    pub date: NaiveDate,
}

impl ModelInfo for GameAvailableDTO {
    const MODEL_NAME: &'static str = "Relation of Game and Location";
    const ID_FIELDS: &'static [&'static str] = &["game id", "location id"];
    const UNIQUE_FIELDS: &'static [&'static str] = GameAvailableDTO::ID_FIELDS;
}
