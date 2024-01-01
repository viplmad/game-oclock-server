use chrono::NaiveDateTime;
use serde::Serialize;
use utoipa::ToSchema;

use super::{DurationDef, GameDTO, GameLogDTO, GameStatus, ModelInfo};

#[derive(Serialize, ToSchema)]
pub struct GameWithLogsDTO {
    pub id: String,
    pub name: String,
    pub edition: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_year: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_filename: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_url: Option<String>,
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
    pub logs: Vec<GameLogDTO>,
}

#[derive(Serialize, ToSchema)]
pub struct GameWithLogDTO {
    pub id: String,
    pub name: String,
    pub edition: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_year: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_filename: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_url: Option<String>,
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
    pub log_start_datetime: NaiveDateTime,
    #[schema(value_type = String, format = DateTime)]
    pub log_end_datetime: NaiveDateTime,
    #[schema(value_type = String)]
    pub log_time: DurationDef,
}

impl ModelInfo for GameWithLogDTO {
    const MODEL_NAME: &'static str = "Game with log";
    const ID_FIELDS: &'static [&'static str] = GameDTO::ID_FIELDS;
    const UNIQUE_FIELDS: &'static [&'static str] = GameDTO::UNIQUE_FIELDS;
}
