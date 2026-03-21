use serde::Serialize;
use utoipa::ToSchema;

use super::{AvailableDTO, LocationDTO, ModelInfo};

#[derive(Serialize, ToSchema)]
pub struct LocationAvailableDTO {
    pub location: LocationDTO,
    pub available: AvailableDTO,
}

impl ModelInfo for LocationAvailableDTO {
    const MODEL_NAME: &'static str = "Relation of Media and Location";
    const ID_FIELDS: &'static [&'static str] = &["media id", "location id"];
    const UNIQUE_FIELDS: &'static [&'static str] = LocationAvailableDTO::ID_FIELDS;
}
