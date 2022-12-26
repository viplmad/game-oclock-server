use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::{Merge, ModelInfo};

#[derive(Serialize, ToSchema)]
pub struct DLCDTO {
    pub id: i32,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_game_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_year: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_filename: Option<String>,
    #[schema(value_type = String, format = DateTime)]
    pub added_datetime: NaiveDateTime,
    #[schema(value_type = String, format = DateTime)]
    pub updated_datetime: NaiveDateTime,
}

impl Default for DLCDTO {
    fn default() -> Self {
        Self {
            id: -1,
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
            name: other.name.unwrap_or(self.name),
            base_game_id: other.base_game_id,
            release_year: other.release_year,
            cover_filename: other.cover_filename,
            added_datetime: self.added_datetime,
            updated_datetime: self.updated_datetime,
        }
    }
}

impl ModelInfo for DLCDTO {
    const MODEL_NAME: &'static str = "DLC";
    const ID_FIELDS: &'static [&'static str] = &["id"];
    const UNIQUE_FIELDS: &'static [&'static str] = &["name"];
}

#[derive(Deserialize, ToSchema)]
pub struct NewDLCDTO {
    pub name: Option<String>,
    pub base_game_id: Option<i32>,
    pub release_year: Option<i32>,
    pub cover_filename: Option<String>,
}

#[derive(Serialize, ToSchema)]
pub struct DLCAvailableDTO {
    pub id: i32,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_game_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_year: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_filename: Option<String>,
    #[schema(value_type = String, format = DateTime)]
    pub added_datetime: NaiveDateTime,
    #[schema(value_type = String, format = DateTime)]
    pub updated_datetime: NaiveDateTime,
    #[schema(value_type = String, format = Date)]
    pub available_date: NaiveDate,
}

impl ModelInfo for DLCAvailableDTO {
    const MODEL_NAME: &'static str = "Relation of DLC and Platform";
    const ID_FIELDS: &'static [&'static str] = &["dlc id", "platform id"];
    const UNIQUE_FIELDS: &'static [&'static str] = DLCAvailableDTO::ID_FIELDS;
}
