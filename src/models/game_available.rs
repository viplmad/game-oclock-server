use chrono::{NaiveDate, NaiveDateTime};
use serde::Serialize;
use utoipa::ToSchema;

use super::{GameStatus, ModelInfo};

#[derive(Serialize, ToSchema)]
pub struct GameAvailableDTO {
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
    #[schema(value_type = String, format = Date)]
    pub available_date: NaiveDate,
}

impl ModelInfo for GameAvailableDTO {
    const MODEL_NAME: &'static str = "Relation of Game and Location";
    const ID_FIELDS: &'static [&'static str] = &["game id", "location id"];
    const UNIQUE_FIELDS: &'static [&'static str] = GameAvailableDTO::ID_FIELDS;
}
