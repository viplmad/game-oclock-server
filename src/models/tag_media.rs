use serde::Serialize;
use utoipa::ToSchema;

use super::{ModelInfo, TagDTO, TaggedDTO};

#[derive(Serialize, ToSchema)]
pub struct TagMediaDTO {
    pub tag: TagDTO,
    pub tagged: TaggedDTO,
}

impl ModelInfo for TagMediaDTO {
    const MODEL_NAME: &'static str = "Relation of Media and Tag";
    const ID_FIELDS: &'static [&'static str] = &["media id", "tag id"];
    const UNIQUE_FIELDS: &'static [&'static str] = TagMediaDTO::ID_FIELDS;
}
