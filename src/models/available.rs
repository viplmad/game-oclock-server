use chrono::{DateTime, FixedOffset};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Default, Serialize, ToSchema)]
pub struct AvailableDTO {
    #[schema(value_type = String, format = DateTime)]
    pub datetime: DateTime<FixedOffset>,
    #[schema(value_type = String, format = DateTime)]
    pub added_datetime: DateTime<FixedOffset>,
    #[schema(value_type = String, format = DateTime)]
    pub updated_datetime: DateTime<FixedOffset>,
}
