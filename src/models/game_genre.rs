use super::ModelInfo;

pub struct GameGenreDTO();

impl ModelInfo for GameGenreDTO {
    const MODEL_NAME: &'static str = "Relation of Game and Genre";
    const ID_FIELDS: &'static [&'static str] = &["game id", "genre id"];
    const UNIQUE_FIELDS: &'static [&'static str] = GameGenreDTO::ID_FIELDS;
}
