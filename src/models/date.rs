use chrono::{DateTime, Utc};
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct DateTimeDTO {
    #[schema(value_type = String, format = DateTime)]
    pub datetime: DateTime<Utc>,
}
