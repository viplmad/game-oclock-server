use chrono::{DateTime, FixedOffset};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Default, Serialize, ToSchema)]
pub struct MediaSessionGroupDTO {
    pub id: Uuid,
    pub name: String,
    #[schema(value_type = String, format = DateTime)]
    pub added_datetime: DateTime<FixedOffset>,
    #[schema(value_type = String, format = DateTime)]
    pub updated_datetime: DateTime<FixedOffset>,
}
