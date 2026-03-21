use chrono::{DateTime, Utc};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Default, Serialize, ToSchema)]
pub struct MediaSessionGroupDTO {
    #[schema(value_type = String)]
    pub id: Uuid,
    pub name: String,
    #[schema(value_type = String, format = DateTime)]
    pub added_datetime: DateTime<Utc>,
    #[schema(value_type = String, format = DateTime)]
    pub updated_datetime: DateTime<Utc>,
}
