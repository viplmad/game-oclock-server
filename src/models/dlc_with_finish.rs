use chrono::{NaiveDate, NaiveDateTime};
use serde::Serialize;
use utoipa::ToSchema;

use super::{ModelInfo, SearchResultDTO, DLCDTO};

pub type DLCWithFinishSearchResult = SearchResultDTO<DLCWithFinishDTO>;

#[derive(Serialize, ToSchema)]
pub struct DLCWithFinishDTO {
    pub id: i32,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_game_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_year: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_filename: Option<String>,
    #[schema(value_type = String)]
    pub added_datetime: NaiveDateTime,
    #[schema(value_type = String)]
    pub updated_datetime: NaiveDateTime,
    #[schema(value_type = String)]
    pub finish_date: NaiveDate,
}

impl ModelInfo for DLCWithFinishDTO {
    const MODEL_NAME: &'static str = "DLC with finish";
    const ID_FIELDS: &'static [&'static str] = DLCDTO::ID_FIELDS;
    const UNIQUE_FIELDS: &'static [&'static str] = DLCDTO::UNIQUE_FIELDS;
}
