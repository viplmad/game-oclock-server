use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use super::{Merge, ModelInfo};

#[derive(Default, Serialize, ToSchema)]
pub struct TagDTO {
    pub id: Uuid,
    pub name: String,
    #[schema(value_type = String, format = DateTime)]
    pub added_datetime: DateTime<FixedOffset>,
    #[schema(value_type = String, format = DateTime)]
    pub updated_datetime: DateTime<FixedOffset>,
}

impl Merge<NewTagDTO> for TagDTO {
    fn merge(self, other: NewTagDTO) -> Self {
        Self {
            id: self.id,
            name: other.name.unwrap_or(self.name),
            added_datetime: self.added_datetime,
            updated_datetime: self.updated_datetime,
        }
    }
}

impl ModelInfo for TagDTO {
    const MODEL_NAME: &'static str = "Tag";
    const ID_FIELDS: &'static [&'static str] = &["id"];
    const UNIQUE_FIELDS: &'static [&'static str] = &["name"];
}

#[derive(Deserialize, ToSchema)]
pub struct NewTagDTO {
    pub name: Option<String>,
}
