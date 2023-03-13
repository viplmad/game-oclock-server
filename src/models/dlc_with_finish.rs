use chrono::{NaiveDate, NaiveDateTime};
use serde::Serialize;
use utoipa::ToSchema;

use super::{ModelInfo, DLCDTO};

#[derive(Serialize, ToSchema)]
pub struct DLCWithFinishDTO {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_game_id: Option<String>,
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
    #[schema(value_type = String, format = Date)]
    pub finish_date: NaiveDate,
}

impl ModelInfo for DLCWithFinishDTO {
    const MODEL_NAME: &'static str = "DLC with finish";
    const ID_FIELDS: &'static [&'static str] = DLCDTO::ID_FIELDS;
    const UNIQUE_FIELDS: &'static [&'static str] = DLCDTO::UNIQUE_FIELDS;
}
