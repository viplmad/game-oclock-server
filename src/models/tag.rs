use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::{Merge, ModelName};

#[derive(Serialize, Deserialize, ToSchema)]
pub struct TagDTO {
    pub id: i32,
    pub user_id: i32, // TODO Remove user id from DTOs
    pub name: String,
    #[schema(value_type = String)]
    pub added_datetime: NaiveDateTime,
    #[schema(value_type = String)]
    pub updated_datetime: NaiveDateTime,
}

impl Default for TagDTO {
    fn default() -> Self {
        Self {
            id: -1,
            user_id: -1,
            name: String::default(),
            added_datetime: NaiveDateTime::default(),
            updated_datetime: NaiveDateTime::default(),
        }
    }
}

impl Merge<NewTagDTO> for TagDTO {
    fn merge(self, other: NewTagDTO) -> Self {
        Self {
            id: self.id,
            user_id: self.user_id,
            name: other.name.unwrap_or(self.name),
            added_datetime: self.added_datetime,
            updated_datetime: self.updated_datetime,
        }
    }
}

impl ModelName for TagDTO {
    const MODEL_NAME: &'static str = "Tag";
    const ID_FIELDS: &'static [&'static str] = &["id"];
    const UNIQUE_FIELDS: &'static [&'static str] = &["name"];
}

#[derive(Deserialize, ToSchema)]
pub struct NewTagDTO {
    pub name: Option<String>,
}
