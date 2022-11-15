use super::ModelName;

pub struct GameTag();

impl ModelName for GameTag {
    const MODEL_NAME: &'static str = "Relation of Game and Tag";
    const ID_FIELDS: &'static [&'static str] = &["game id", "tag id"];
    const UNIQUE_FIELDS: &'static [&'static str] = GameTag::ID_FIELDS;
}
