use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::{Merge, ModelInfo};

#[derive(Default, Serialize, ToSchema)]
pub struct LocationDTO {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_filename: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[schema(value_type = String, format = DateTime)]
    pub added_datetime: NaiveDateTime,
    #[schema(value_type = String, format = DateTime)]
    pub updated_datetime: NaiveDateTime,
}

impl Merge<NewLocationDTO> for LocationDTO {
    fn merge(self, other: NewLocationDTO) -> Self {
        Self {
            id: self.id,
            name: other.name.unwrap_or(self.name),
            icon_filename: self.icon_filename,
            icon_url: self.icon_url,
            added_datetime: self.added_datetime,
            updated_datetime: self.updated_datetime,
        }
    }
}

impl ModelInfo for LocationDTO {
    const MODEL_NAME: &'static str = "Location";
    const ID_FIELDS: &'static [&'static str] = &["id"];
    const UNIQUE_FIELDS: &'static [&'static str] = &["name"];
}

#[derive(Deserialize, ToSchema)]
pub struct NewLocationDTO {
    pub name: Option<String>,
}
