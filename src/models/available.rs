use chrono::{DateTime, Utc};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Default, Serialize, ToSchema)]
pub struct AvailableDTO {
    #[schema(value_type = String, format = DateTime)]
    pub datetime: DateTime<Utc>,
    #[schema(value_type = String, format = DateTime)]
    pub added_datetime: DateTime<Utc>,
    #[schema(value_type = String, format = DateTime)]
    pub updated_datetime: DateTime<Utc>,
}
