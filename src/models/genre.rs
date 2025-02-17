use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use super::{Merge, ModelInfo};

#[derive(Default, Serialize, ToSchema)]
pub struct GenreDTO {
    #[schema(value_type = String)]
    pub id: Uuid,
    #[schema(value_type = String)]
    pub user_id: Uuid,
    pub name: String,
    #[schema(value_type = String, format = DateTime)]
    pub added_datetime: NaiveDateTime,
    #[schema(value_type = String, format = DateTime)]
    pub updated_datetime: NaiveDateTime,
}

impl Merge<NewGenreDTO> for GenreDTO {
    fn merge(self, other: NewGenreDTO) -> Self {
        Self {
            id: self.id,
            user_id: self.user_id,
            name: other.name.unwrap_or(self.name),
            added_datetime: self.added_datetime,
            updated_datetime: self.updated_datetime,
        }
    }
}

impl ModelInfo for GenreDTO {
    const MODEL_NAME: &'static str = "Genre";
    const ID_FIELDS: &'static [&'static str] = &["id"];
    const UNIQUE_FIELDS: &'static [&'static str] = &["name"];
}

#[derive(Deserialize, ToSchema)]
pub struct NewGenreDTO {
    pub name: Option<String>,
}
