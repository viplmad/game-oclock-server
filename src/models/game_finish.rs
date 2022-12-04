use super::ModelInfo;

pub struct GameFinish();

impl ModelInfo for GameFinish {
    const MODEL_NAME: &'static str = "Game finish";
    const ID_FIELDS: &'static [&'static str] = &["game id", "date"];
    const UNIQUE_FIELDS: &'static [&'static str] = GameFinish::ID_FIELDS;
}
