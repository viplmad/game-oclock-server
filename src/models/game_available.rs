use chrono::{NaiveDate, NaiveDateTime};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use super::{GameStatus, ModelInfo};

#[derive(Serialize, ToSchema)]
pub struct GameAvailableDTO {
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
    #[schema(value_type = String, format = Date)]
    pub available_date: NaiveDate,
}

impl ModelInfo for GameAvailableDTO {
    const MODEL_NAME: &'static str = "Relation of Game and Location";
    const ID_FIELDS: &'static [&'static str] = &["game id", "location id"];
    const UNIQUE_FIELDS: &'static [&'static str] = GameAvailableDTO::ID_FIELDS;
}
