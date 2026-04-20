use chrono::NaiveDate;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, ToSchema)]
pub struct SessionStreakDTO {
    #[schema(value_type = String, format = Date)]
    pub start_date: NaiveDate,
    #[schema(value_type = String, format = Date)]
    pub end_date: NaiveDate,
    pub days: u32,
    #[schema(value_type = String)]
    pub devices_ids: Vec<Uuid>,
}

#[derive(Serialize, ToSchema)]
pub struct MediasSessionStreakDTO {
    #[schema(value_type = String)]
    pub medias_ids: Vec<Uuid>,
    pub streak: SessionStreakDTO,
}
