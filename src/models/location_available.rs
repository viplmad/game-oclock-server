use chrono::NaiveDate;
use serde::Serialize;
use utoipa::ToSchema;

use super::{LocationDTO, ModelInfo};

#[derive(Serialize, ToSchema)]
pub struct LocationAvailableDTO {
    pub location: LocationDTO,
    #[schema(value_type = String, format = Date)]
    pub date: NaiveDate,
}

impl ModelInfo for LocationAvailableDTO {
    const MODEL_NAME: &'static str = "Relation with Location";
    const ID_FIELDS: &'static [&'static str] = &["id", "location id"];
    const UNIQUE_FIELDS: &'static [&'static str] = LocationAvailableDTO::ID_FIELDS;
}
