use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::ModelInfo;

#[derive(Default, Serialize, Deserialize, ToSchema)]
pub struct ExternalMediaIdDTO {
    pub source: String,
    pub id: String,
}

impl ModelInfo for ExternalMediaIdDTO {
    const MODEL_NAME: &'static str = "ExternalMediaId";
    const ID_FIELDS: &'static [&'static str] = &["source", "id"];
    const UNIQUE_FIELDS: &'static [&'static str] = &["source", "id"];
}
