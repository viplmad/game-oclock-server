use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use super::{GameStatus, Merge, ModelInfo};

#[derive(Default, Serialize, ToSchema)]
pub struct GameDTO {
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
    pub rating: i16,
    pub notes: String,
}

impl Merge<NewGameDTO> for GameDTO {
    fn merge(self, other: NewGameDTO) -> Self {
        Self {
            id: self.id,
            user_id: self.user_id,
            title: other.title.unwrap_or(self.title),
            edition: other.edition.unwrap_or(self.edition),
            release_date: other.release_date,
            base_game_id: self.base_game_id,
            cover_url: other.cover_url,
            added_datetime: self.added_datetime,
            updated_datetime: self.updated_datetime,
            status: other.status.unwrap_or(self.status),
            rating: other.rating.unwrap_or(self.rating),
            notes: other.notes.unwrap_or(self.notes),
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
    pub title: Option<String>,
    pub edition: Option<String>,
    pub release_date: Option<NaiveDate>,
    pub cover_url: Option<String>,
    pub status: Option<GameStatus>,
    pub rating: Option<i16>,
    pub notes: Option<String>,
}
