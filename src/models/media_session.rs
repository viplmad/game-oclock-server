use serde::Serialize;
use utoipa::ToSchema;

use super::{MediaDTO, ModelInfo, SessionDTO};

#[derive(Serialize, ToSchema)]
pub struct MediaSessionDTO {
    pub media: MediaDTO,
    pub session: SessionDTO,
}

impl ModelInfo for MediaSessionDTO {
    const MODEL_NAME: &'static str = "Relation of Media and Session";
    const ID_FIELDS: &'static [&'static str] = &["media id", "session id"];
    const UNIQUE_FIELDS: &'static [&'static str] = MediaSessionDTO::ID_FIELDS;
}
