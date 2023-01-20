use chrono::{NaiveDate, NaiveDateTime};
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct DateDTO {
    #[schema(value_type = String, format = Date)]
    pub date: NaiveDate,
}

#[derive(Deserialize, ToSchema)]
pub struct DateTimeDTO {
    #[schema(value_type = String, format = DateTime)]
    pub datetime: NaiveDateTime,
}
