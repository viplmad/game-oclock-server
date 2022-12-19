use chrono::NaiveDate;
use serde::Deserialize;
use utoipa::IntoParams;

#[derive(Deserialize, IntoParams)]
#[into_params(names("id"))]
pub struct ItemId(pub i32);

#[derive(Deserialize, IntoParams)]
#[into_params(names("id", "other_id"))]
pub struct ItemIdAndRelatedId(pub i32, pub i32);

#[derive(Deserialize, IntoParams)]
pub struct StartEndDateQuery {
    #[param(value_type = String)]
    pub start_date: NaiveDate,
    #[param(value_type = String)]
    pub end_date: NaiveDate,
}
