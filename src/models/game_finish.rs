use super::ModelInfo;

pub struct GameFinishDTO();

impl ModelInfo for GameFinishDTO {
    const MODEL_NAME: &'static str = "Game finish";
    const ID_FIELDS: &'static [&'static str] = &["game id", "date"];
    const UNIQUE_FIELDS: &'static [&'static str] = GameFinishDTO::ID_FIELDS;
}
