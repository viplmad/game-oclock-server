use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::models::ModelInfo;

#[derive(Deserialize, Serialize, ToSchema)]
pub struct SessionStreakDTO {
    #[schema(value_type = String, format = Date)]
    pub start_date: NaiveDate,
    #[schema(value_type = String, format = Date)]
    pub end_date: NaiveDate,
    pub days: u32,
    pub media_ids: Vec<Uuid>,
    pub device_ids: Vec<Uuid>,
}

impl ModelInfo for SessionStreakDTO {
    const MODEL_NAME: &'static str = "Media session";
    const ID_FIELDS: &'static [&'static str] = &["media id", "start datetime"];
    const UNIQUE_FIELDS: &'static [&'static str] = SessionStreakDTO::ID_FIELDS;
}
