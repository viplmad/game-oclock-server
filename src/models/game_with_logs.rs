use chrono::NaiveDateTime;
use serde::Serialize;
use utoipa::ToSchema;

use super::{GameLogDTO, GameStatus};

#[derive(Serialize, ToSchema)]
pub struct GameWithLogsDTO {
    pub id: i32,
    pub name: String,
    pub edition: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_year: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_filename: Option<String>,
    #[schema(value_type = String)]
    pub added_datetime: NaiveDateTime,
    #[schema(value_type = String)]
    pub updated_datetime: NaiveDateTime,
    pub status: GameStatus,
    pub rating: i32,
    pub notes: String,
    pub save_folder: String,
    pub screenshot_folder: String,
    pub backup: bool,
    pub logs: Vec<GameLogDTO>,
}
