use chrono::{NaiveDate, NaiveDateTime};
use serde::Serialize;
use utoipa::ToSchema;

use super::{GameDTO, GameStatus, ModelInfo, SearchResultDTO};

pub type GameWithFinishSearchResult = SearchResultDTO<GameWithFinishDTO>;

#[derive(Serialize, ToSchema)]
pub struct GameWithFinishDTO {
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
    pub finish_date: NaiveDate,
}

impl ModelInfo for GameWithFinishDTO {
    const MODEL_NAME: &'static str = "Game with finish";
    const ID_FIELDS: &'static [&'static str] = GameDTO::ID_FIELDS;
    const UNIQUE_FIELDS: &'static [&'static str] = GameDTO::UNIQUE_FIELDS;
}
