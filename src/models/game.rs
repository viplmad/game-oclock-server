use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::{GameStatus, Merge, ModelInfo};

#[derive(Serialize, ToSchema)]
pub struct GameDTO {
    pub id: i32,
    pub name: String,
    pub edition: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_year: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_filename: Option<String>,
    #[schema(value_type = String, format = DateTime)]
    pub added_datetime: NaiveDateTime,
    #[schema(value_type = String, format = DateTime)]
    pub updated_datetime: NaiveDateTime,
    pub status: GameStatus,
    pub rating: i32,
    pub notes: String,
    pub save_folder: String,
    pub screenshot_folder: String,
    pub backup: bool,
}

impl Default for GameDTO {
    fn default() -> Self {
        Self {
            id: -1,
            name: String::default(),
            edition: String::default(),
            release_year: None,
            cover_filename: None,
            added_datetime: NaiveDateTime::default(),
            updated_datetime: NaiveDateTime::default(),
            status: GameStatus::default(),
            rating: 0,
            notes: String::default(),
            save_folder: String::default(),
            screenshot_folder: String::default(),
            backup: false,
        }
    }
}

impl Merge<NewGameDTO> for GameDTO {
    fn merge(self, other: NewGameDTO) -> Self {
        Self {
            id: self.id,
            name: other.name.unwrap_or(self.name),
            edition: other.edition.unwrap_or(self.edition),
            release_year: other.release_year,
            cover_filename: other.cover_filename,
            added_datetime: self.added_datetime,
            updated_datetime: self.updated_datetime,
            status: other.status.unwrap_or(self.status),
            rating: other.rating.unwrap_or(self.rating),
            notes: other.notes.unwrap_or(self.notes),
            save_folder: other.save_folder.unwrap_or(self.save_folder),
            screenshot_folder: other.screenshot_folder.unwrap_or(self.screenshot_folder),
            backup: other.backup.unwrap_or(self.backup),
        }
    }
}

impl ModelInfo for GameDTO {
    const MODEL_NAME: &'static str = "Game";
    const ID_FIELDS: &'static [&'static str] = &["id"];
    const UNIQUE_FIELDS: &'static [&'static str] = &["name", "edition"];
}

#[derive(Deserialize, ToSchema)]
pub struct NewGameDTO {
    pub name: Option<String>,
    pub edition: Option<String>,
    pub release_year: Option<i32>,
    pub cover_filename: Option<String>,
    pub status: Option<GameStatus>,
    pub rating: Option<i32>,
    pub notes: Option<String>,
    pub save_folder: Option<String>,
    pub screenshot_folder: Option<String>,
    pub backup: Option<bool>,
}

#[derive(Serialize, ToSchema)]
pub struct GameAvailableDTO {
    pub id: i32,
    pub name: String,
    pub edition: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_year: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_filename: Option<String>,
    #[schema(value_type = String, format = DateTime)]
    pub added_datetime: NaiveDateTime,
    #[schema(value_type = String, format = DateTime)]
    pub updated_datetime: NaiveDateTime,
    pub status: GameStatus,
    pub rating: i32,
    pub notes: String,
    pub save_folder: String,
    pub screenshot_folder: String,
    pub backup: bool,
    #[schema(value_type = String, format = DateTime)]
    pub available_date: NaiveDate,
}

impl ModelInfo for GameAvailableDTO {
    const MODEL_NAME: &'static str = "Relation of Game and Platform";
    const ID_FIELDS: &'static [&'static str] = &["game id", "platform id"];
    const UNIQUE_FIELDS: &'static [&'static str] = GameAvailableDTO::ID_FIELDS;
}
