use super::ModelInfo;

pub struct GameTagDTO();

impl ModelInfo for GameTagDTO {
    const MODEL_NAME: &'static str = "Relation of Game and Tag";
    const ID_FIELDS: &'static [&'static str] = &["game id", "tag id"];
    const UNIQUE_FIELDS: &'static [&'static str] = GameTagDTO::ID_FIELDS;
}
