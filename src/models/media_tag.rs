use serde::Serialize;
use utoipa::ToSchema;

use super::{MediaDTO, ModelInfo, TaggedDTO};

#[derive(Serialize, ToSchema)]
pub struct MediaTagDTO {
    pub media: MediaDTO,
    pub tagged: TaggedDTO,
}

impl ModelInfo for MediaTagDTO {
    const MODEL_NAME: &'static str = "Relation of Media and Tag";
    const ID_FIELDS: &'static [&'static str] = &["media id", "tag id"];
    const UNIQUE_FIELDS: &'static [&'static str] = MediaTagDTO::ID_FIELDS;
}
