use serde::Serialize;
use utoipa::ToSchema;

use super::{AvailableDTO, MediaDTO, ModelInfo};

#[derive(Serialize, ToSchema)]
pub struct MediaAvailableDTO {
    pub media: MediaDTO,
    pub available: AvailableDTO,
}

impl ModelInfo for MediaAvailableDTO {
    const MODEL_NAME: &'static str = "Relation of Media and Location";
    const ID_FIELDS: &'static [&'static str] = &["media id", "location id"];
    const UNIQUE_FIELDS: &'static [&'static str] = MediaAvailableDTO::ID_FIELDS;
}
