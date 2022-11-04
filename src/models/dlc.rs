use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::{Merge, ModelName};

#[derive(Serialize, Deserialize, ToSchema)]
pub struct DLCDTO {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub base_game_id: Option<i32>,
    pub release_year: Option<i32>,
    pub cover_filename: Option<String>,
    #[schema(value_type = String)]
    pub added_datetime: NaiveDateTime,
    #[schema(value_type = String)]
    pub updated_datetime: NaiveDateTime,
}

impl Default for DLCDTO {
    fn default() -> Self {
        Self {
            id: -1,
            user_id: -1,
            name: String::default(),
            base_game_id: None,
            release_year: None,
            cover_filename: None,
            added_datetime: NaiveDateTime::default(),
            updated_datetime: NaiveDateTime::default(),
        }
    }
}

impl Merge<NewDLCDTO> for DLCDTO {
    fn merge(self, other: NewDLCDTO) -> Self {
        Self {
            id: self.id,
            user_id: self.user_id,
            name: other.name.unwrap_or(self.name),
            base_game_id: other.base_game_id,
            release_year: other.release_year,
            cover_filename: other.cover_filename,
            added_datetime: self.added_datetime,
            updated_datetime: self.updated_datetime,
        }
    }
}

impl ModelName for DLCDTO {
    const MODEL_NAME: &'static str = "DLC";
    const ID_FIELDS: &'static [&'static str] = &["id"];
    const UNIQUE_FIELDS: &'static [&'static str] = &["name"];
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct NewDLCDTO {
    pub name: Option<String>,
    pub base_game_id: Option<i32>,
    pub release_year: Option<i32>,
    pub cover_filename: Option<String>,
}
