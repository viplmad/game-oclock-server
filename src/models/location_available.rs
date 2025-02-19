use chrono::{NaiveDate, NaiveDateTime};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use super::ModelInfo;

#[derive(Serialize, ToSchema)]
pub struct LocationAvailableDTO {
    #[schema(value_type = String)]
    pub id: Uuid,
    #[schema(value_type = String)]
    pub user_id: Uuid,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[schema(value_type = String, format = DateTime)]
    pub added_datetime: NaiveDateTime,
    #[schema(value_type = String, format = DateTime)]
    pub updated_datetime: NaiveDateTime,
    #[schema(value_type = String, format = Date)]
    pub date: NaiveDate,
}

impl ModelInfo for LocationAvailableDTO {
    const MODEL_NAME: &'static str = "Relation with Location";
    const ID_FIELDS: &'static [&'static str] = &["id", "location id"];
    const UNIQUE_FIELDS: &'static [&'static str] = LocationAvailableDTO::ID_FIELDS;
}
