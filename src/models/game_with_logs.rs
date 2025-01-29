use chrono::{NaiveDate, NaiveDateTime};
use serde::Serialize;
use utoipa::ToSchema;

use super::{DurationDef, GameDTO, GameStatus, LogDTO, ModelInfo};

// TODO Remove - create calls for aggregation
#[derive(Serialize, ToSchema)]
pub struct GameWithLogsDTO {
    pub id: String,
    pub title: String,
    pub edition: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_date: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_game_id: Option<String>,
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
    pub logs: Vec<LogDTO>,
}

#[derive(Serialize, ToSchema)]
pub struct GameWithLogDTO {
    pub id: String,
    pub title: String,
    pub edition: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_date: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_game_id: Option<String>,
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
    #[schema(value_type = String, format = DateTime)]
    pub log_start_datetime: NaiveDateTime,
    #[schema(value_type = String, format = DateTime)]
    pub log_end_datetime: NaiveDateTime,
    pub log_device_id: String,
    #[schema(value_type = String)]
    pub log_time: DurationDef,
}

impl ModelInfo for GameWithLogDTO {
    const MODEL_NAME: &'static str = "Game with log";
    const ID_FIELDS: &'static [&'static str] = GameDTO::ID_FIELDS;
    const UNIQUE_FIELDS: &'static [&'static str] = GameDTO::UNIQUE_FIELDS;
}
