use chrono::{NaiveDate, NaiveDateTime};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use super::{DurationDef, GameDTO, GameStatus, ModelInfo};

#[derive(Serialize, ToSchema)]
pub struct GameWithLogDTO {
    #[schema(value_type = String)]
    pub id: Uuid,
    #[schema(value_type = String)]
    pub user_id: Uuid,
    pub title: String,
    pub edition: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_date: Option<NaiveDate>,
    #[schema(value_type = String)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_game_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_url: Option<String>,
    #[schema(value_type = String, format = DateTime)]
    pub added_datetime: NaiveDateTime,
    #[schema(value_type = String, format = DateTime)]
    pub updated_datetime: NaiveDateTime,
    pub status: GameStatus,
    pub rating: u32,
    pub notes: String,
    #[schema(value_type = String, format = DateTime)]
    pub log_start_datetime: NaiveDateTime, // TODO Pass as typed field
    #[schema(value_type = String, format = DateTime)]
    pub log_end_datetime: NaiveDateTime,
    #[schema(value_type = String)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_device_id: Option<Uuid>,
    #[schema(value_type = String)]
    pub log_time: DurationDef,
}

impl ModelInfo for GameWithLogDTO {
    const MODEL_NAME: &'static str = "Game with log";
    const ID_FIELDS: &'static [&'static str] = GameDTO::ID_FIELDS;
    const UNIQUE_FIELDS: &'static [&'static str] = GameDTO::UNIQUE_FIELDS;
}
